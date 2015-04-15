// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use self::ArgumentType::*;
use self::Position::*;

use ast;
use codemap::{Span, respan};
use ext::base::*;
use ext::base;
use ext::build::AstBuilder;
use fmt_macros as parse;
use fold::Folder;
use parse::token::special_idents;
use parse::token;
use ptr::P;

use std::collections::HashMap;
use std::iter::repeat;

#[derive(PartialEq)]
enum ArgumentType {
    Known(String),
    Unsigned
}

enum Position {
    Exact(usize),
    Named(String),
}

struct Context<'a, 'b:'a> {
    ecx: &'a mut ExtCtxt<'b>,
    fmtsp: Span,

    writer: P<ast::Expr>,

    /// Parsed argument expressions and the types that we've found so far for
    /// them.
    args: Vec<P<ast::Expr>>,
    arg_types: Vec<Option<ArgumentType>>,
    /// Parsed named expressions and the types that we've found for them so far.
    /// Note that we keep a side-array of the ordering of the named arguments
    /// found to be sure that we can translate them in the same order that they
    /// were declared in.
    names: HashMap<String, P<ast::Expr>>,
    name_types: HashMap<String, ArgumentType>,
    name_ordering: Vec<String>,

    /// The latest consecutive literal strings, or empty if there weren't any.
    literal: String,

    /// Collection of the compiled `rt::Argument` structures
    pieces: Vec<P<ast::Expr>>,

    name_positions: HashMap<String, usize>,

    /// Updated as arguments are consumed or methods are entered
    nest_level: usize,
    next_arg: usize,
}

/// Parses the arguments from the given list of tokens, returning None
/// if there's a parse error so we can continue parsing other format!
/// expressions.
///
/// If parsing succeeds, the return value is:
///
///     Some((writer, fmtstr, unnamed arguments, ordering of named arguments,
///           named arguments))
fn parse_args(ecx: &mut ExtCtxt, sp: Span, tts: &[ast::TokenTree])
              -> Option<(P<ast::Expr>, P<ast::Expr>, Vec<P<ast::Expr>>, Vec<String>,
                         HashMap<String, P<ast::Expr>>)> {
    let mut args = Vec::new();
    let mut names = HashMap::<String, P<ast::Expr>>::new();
    let mut order = Vec::new();

    let mut p = ecx.new_parser_from_tts(tts);

    // parse the writer
    if p.token == token::Eof {
        ecx.span_err(sp, "requires a writer");
        return None;
    }
    let writer = p.parse_expr();

    if !panictry!(p.eat(&token::Comma)) {
        ecx.span_err(sp, "expected token: `,`");
        return None;
    }

    // parse the format string
    if p.token == token::Eof {
        ecx.span_err(sp, "requires a writer");
        return None;
    }
    let fmtstr = p.parse_expr();

    let mut named = false;
    while p.token != token::Eof {
        if !panictry!(p.eat(&token::Comma)) {
            ecx.span_err(sp, "expected token: `,`");
            return None;
        }
        if p.token == token::Eof { break } // accept trailing commas
        if named || (p.token.is_ident() && p.look_ahead(1, |t| *t == token::Eq)) {
            named = true;
            let ident = match p.token {
                token::Ident(i, _) => {
                    panictry!(p.bump());
                    i
                }
                _ if named => {
                    ecx.span_err(p.span,
                                 "expected ident, positional arguments \
                                 cannot follow named arguments");
                    return None;
                }
                _ => {
                    ecx.span_err(p.span,
                                 &format!("expected ident for named argument, found `{}`",
                                         p.this_token_to_string()));
                    return None;
                }
            };
            let interned_name = token::get_ident(ident);
            let name = &interned_name[..];

            panictry!(p.expect(&token::Eq));
            let e = p.parse_expr();
            match names.get(name) {
                None => {}
                Some(prev) => {
                    ecx.span_err(e.span,
                                 &format!("duplicate argument named `{}`",
                                         name));
                    ecx.parse_sess.span_diagnostic.span_note(prev.span, "previously here");
                    continue
                }
            }
            order.push(name.to_string());
            names.insert(name.to_string(), e);
        } else {
            args.push(p.parse_expr());
        }
    }
    Some((writer, fmtstr, args, order, names))
}

impl<'a, 'b> Context<'a, 'b> {
    /// Verifies one piece of a parse string. All errors are not emitted as
    /// fatal so we can continue giving errors about this and possibly other
    /// format strings.
    fn verify_piece(&mut self, p: &parse::Piece) {
        match *p {
            parse::String(..) => {}
            parse::NextArgument(ref arg) => {
                // width/precision first, if they have implicit positional
                // parameters it makes more sense to consume them first.
                self.verify_count(arg.format.width);
                self.verify_count(arg.format.precision);

                // argument second, if it's an implicit positional parameter
                // it's written second, so it should come after width/precision.
                let pos = match arg.position {
                    parse::ArgumentNext => {
                        let i = self.next_arg;
                        if self.check_positional_ok() {
                            self.next_arg += 1;
                        }
                        Exact(i)
                    }
                    parse::ArgumentIs(i) => Exact(i),
                    parse::ArgumentNamed(s) => Named(s.to_string()),
                };

                let ty = Known(arg.format.ty.to_string());
                self.verify_arg_type(pos, ty);
            }
        }
    }

    fn verify_count(&mut self, c: parse::Count) {
        match c {
            parse::CountImplied | parse::CountIs(..) => {}
            parse::CountIsParam(i) => {
                self.verify_arg_type(Exact(i), Unsigned);
            }
            parse::CountIsName(s) => {
                self.verify_arg_type(Named(s.to_string()), Unsigned);
            }
            parse::CountIsNextParam => {
                if self.check_positional_ok() {
                    let next_arg = self.next_arg;
                    self.verify_arg_type(Exact(next_arg), Unsigned);
                    self.next_arg += 1;
                }
            }
        }
    }

    fn check_positional_ok(&mut self) -> bool {
        if self.nest_level != 0 {
            self.ecx.span_err(self.fmtsp, "cannot use implicit positional \
                                           arguments nested inside methods");
            false
        } else {
            true
        }
    }

    fn describe_num_args(&self) -> String {
        match self.args.len() {
            0 => "no arguments given".to_string(),
            1 => "there is 1 argument".to_string(),
            x => format!("there are {} arguments", x),
        }
    }

    fn verify_arg_type(&mut self, arg: Position, ty: ArgumentType) {
        match arg {
            Exact(arg) => {
                if self.args.len() <= arg {
                    let msg = format!("invalid reference to argument `{}` ({})",
                                      arg, self.describe_num_args());

                    self.ecx.span_err(self.fmtsp, &msg[..]);
                    return;
                }
                {
                    let arg_type = match self.arg_types[arg] {
                        None => None,
                        Some(ref x) => Some(x)
                    };
                    self.verify_same(self.args[arg].span, &ty, arg_type);
                }
                if self.arg_types[arg].is_none() {
                    self.arg_types[arg] = Some(ty);
                }
            }

            Named(name) => {
                let span = match self.names.get(&name) {
                    Some(e) => e.span,
                    None => {
                        let msg = format!("there is no argument named `{}`", name);
                        self.ecx.span_err(self.fmtsp, &msg[..]);
                        return;
                    }
                };
                self.verify_same(span, &ty, self.name_types.get(&name));
                if !self.name_types.contains_key(&name) {
                    self.name_types.insert(name.clone(), ty);
                }
                // Assign this named argument a slot in the arguments array if
                // it hasn't already been assigned a slot.
                if !self.name_positions.contains_key(&name) {
                    let slot = self.name_positions.len();
                    self.name_positions.insert(name, slot);
                }
            }
        }
    }

    /// When we're keeping track of the types that are declared for certain
    /// arguments, we assume that `None` means we haven't seen this argument
    /// yet, `Some(None)` means that we've seen the argument, but no format was
    /// specified, and `Some(Some(x))` means that the argument was declared to
    /// have type `x`.
    ///
    /// Obviously `Some(Some(x)) != Some(Some(y))`, but we consider it true
    /// that: `Some(None) == Some(Some(x))`
    fn verify_same(&self,
                   sp: Span,
                   ty: &ArgumentType,
                   before: Option<&ArgumentType>) {
        let cur = match before {
            None => return,
            Some(t) => t,
        };
        if *ty == *cur {
            return
        }
        match (cur, ty) {
            (&Known(ref cur), &Known(ref ty)) => {
                self.ecx.span_err(sp,
                                  &format!("argument redeclared with type `{}` when \
                                           it was previously `{}`",
                                          *ty,
                                          *cur));
            }
            (&Known(ref cur), _) => {
                self.ecx.span_err(sp,
                                  &format!("argument used to format with `{}` was \
                                           attempted to not be used for formatting",
                                           *cur));
            }
            (_, &Known(ref ty)) => {
                self.ecx.span_err(sp,
                                  &format!("argument previously used as a format \
                                           argument attempted to be used as `{}`",
                                           *ty));
            }
            (_, _) => {
                self.ecx.span_err(sp, "argument declared with multiple formats");
            }
        }
    }

    /// Translate the accumulated string literals to a literal expression
    fn trans_literal_string(&mut self) -> P<ast::Expr> {
        let s = token::intern_and_get_ident(&self.literal);
        self.literal.clear();

        // ::fmt::Debug::fmt(arg, writer)

        let call = {
            let path = vec!(self.ecx.ident_of("fmt"),
                            self.ecx.ident_of("Display"),
                            self.ecx.ident_of("fmt"));

            let arg = self.ecx.expr_str(self.fmtsp, s);
            let writer = self.ecx.expr_ident(self.fmtsp, self.ecx.ident_of("writer"));

            self.ecx.expr_call_global(self.fmtsp, path, vec!(arg, writer))
        };

        self.trans_common(call)
    }

    /// Translate a `parse::Piece` to a ... or append to the `literal` string.
    fn trans_piece(&mut self, piece: &parse::Piece) -> Option<P<ast::Expr>> {
        let arg = match *piece {
            parse::String(s) => {
                self.literal.push_str(s);
                return None;
            }
            parse::NextArgument(ref arg) => arg,
        };

        // Debug/Display/...

        let trait_name = match arg.format.ty {
            ""  => "Display",
            "?" => "Debug",
            "x" => "LowerHex",
            "X" => "UpperHex",
            x => {
                self.ecx.span_err(self.fmtsp, &format!("unknown format trait `{}`", x));
                "Dummy"
            }
        };

        // ::fmt::Debug::fmt(arg, writer)

        let call = {
            let path = vec!(self.ecx.ident_of("linux"),
                            self.ecx.ident_of("fmt"),
                            self.ecx.ident_of(trait_name),
                            self.ecx.ident_of("fmt"));

            let arg_name = &format!("arg{}", self.next_arg - 1);
            let arg = self.ecx.expr_ident(self.fmtsp, self.ecx.ident_of(arg_name));
            let writer = self.ecx.expr_ident(self.fmtsp, self.ecx.ident_of("writer"));

            self.ecx.expr_call_global(self.fmtsp, path, vec!(arg, writer))
        };

        Some(self.trans_common(call))
    }

    fn trans_common(&mut self, call: P<ast::Expr>) -> P<ast::Expr> {

        // res = call

        let assign = {
            let res = self.ecx.expr_ident(self.fmtsp, self.ecx.ident_of("res"));
            self.ecx.stmt_expr(self.ecx.expr(self.fmtsp, ast::ExprAssign(res, call)))
        };

        // res.is_err()

        let is_err = {
            let res = self.ecx.expr_ident(self.fmtsp, self.ecx.ident_of("res"));
            self.ecx.expr_method_call(self.fmtsp, res, self.ecx.ident_of("is_err"), vec!())
        };

        // { assign; is_err }

        let block = {
            self.ecx.expr_block(self.ecx.block(self.fmtsp, vec!(assign), Some(is_err)))
        };

        block
    }

    fn static_array(ecx: &mut ExtCtxt,
                    name: &str,
                    piece_ty: P<ast::Ty>,
                    pieces: Vec<P<ast::Expr>>)
                    -> P<ast::Expr> {
        let fmtsp = piece_ty.span;
        let ty = ecx.ty_rptr(fmtsp,
            ecx.ty(fmtsp, ast::TyVec(piece_ty)),
            Some(ecx.lifetime(fmtsp, special_idents::static_lifetime.name)),
            ast::MutImmutable);
        let slice = ecx.expr_vec_slice(fmtsp, pieces);
        let st = ast::ItemStatic(ty, ast::MutImmutable, slice);

        let name = ecx.ident_of(name);
        let item = ecx.item(fmtsp, name, vec![], st);
        let decl = respan(fmtsp, ast::DeclItem(item));

        // Wrap the declaration in a block so that it forms a single expression.
        ecx.expr_block(ecx.block(fmtsp,
            vec![P(respan(fmtsp, ast::StmtDecl(P(decl), ast::DUMMY_NODE_ID)))],
            Some(ecx.expr_ident(fmtsp, name))))
    }

    /// Actually builds the expression which the iformat! block will be expanded
    /// to
    fn into_expr(mut self) -> P<ast::Expr> {
        if self.pieces.len() == 0 {
            let path = vec!(self.ecx.ident_of("linux"),
                            self.ecx.ident_of("option"),
                            self.ecx.ident_of("Option"),
                            self.ecx.ident_of("Ok"));
            let unit =  self.ecx.expr_tuple(self.fmtsp, vec!());

            return self.ecx.expr_call_global(self.fmtsp, path, vec!(unit));
        }

        // let mut res;

        let let_mut_res = {
            let ident = self.ecx.ident_of("res");
            self.stmt_let(self.fmtsp, true, Some(ident), None)
        };

        // piece1 || piece2 || ...

        let p1_p2 = {
            let mut ors = self.pieces[0].clone();
            for piece in &self.pieces[1..] {
                ors = self.ecx.expr_binary(self.fmtsp, ast::BiOr, ors, piece.clone());
            }
            ors
        };

        // let _ = ...

        let wild_let = {
            self.stmt_let(self.fmtsp, false, None, Some(p1_p2))
        };

        // res

        let res = {
            self.ecx.expr_ident(self.fmtsp, self.ecx.ident_of("res"))
        };

        // { let_mut_res; wild_let; res }

        let match_block = {
            let block = self.ecx.block(self.fmtsp, vec!(let_mut_res, wild_let), Some(res));
            self.ecx.expr_block(block)
        };

        // (writer, arg0, arg1) =>

        let pattern = {
            let mut pats = vec!();
            pats.push(self.ecx.pat_ident(self.fmtsp, self.ecx.ident_of("writer")));
            for n in 0..self.args.len() {
                let name = &format!("arg{}", n);
                pats.push(self.ecx.pat_ident(self.fmtsp, self.ecx.ident_of(name)));
            }
            self.ecx.pat_tuple(self.fmtsp, pats)
        };

        // patters => match_block

        let arm = self.ecx.arm(self.fmtsp, vec!(pattern), match_block);

        // (&mut [], &1, &"test")

        let match_tuple = {
            let mut vec = vec!();
            vec.push(self.ecx.expr_mut_addr_of(self.fmtsp, self.writer.clone()));
            for arg in &self.args {
                vec.push(self.ecx.expr_addr_of(self.fmtsp, arg.clone()));
            }
            self.ecx.expr_tuple(self.fmtsp, vec)
        };

        // match match_tuple { ... }

        let match_expr = {
            self.ecx.expr_match(self.fmtsp, match_tuple, vec!(arm))
        };

        match_expr
    }

    fn stmt_let(&self, sp: Span, mutbl: bool, ident: Option<ast::Ident>,
                ex: Option<P<ast::Expr>>) -> P<ast::Stmt> {
        let pat = match (ident, mutbl) {
            (Some(id), true) =>
                self.ecx.pat_ident_binding_mode(sp, id, ast::BindByValue(ast::MutMutable)),
            (Some(id), false) => self.ecx.pat_ident(sp, id),
            _ => self.ecx.pat_wild(sp),
        };
        let local = P(ast::Local {
            pat: pat,
            ty: None,
            init: ex,
            id: ast::DUMMY_NODE_ID,
            span: sp,
            source: ast::LocalLet,
        });
        let decl = respan(sp, ast::DeclLocal(local));
        P(respan(sp, ast::StmtDecl(P(decl), ast::DUMMY_NODE_ID)))
    }
}

pub fn expand_format_args<'cx>(ecx: &'cx mut ExtCtxt, sp: Span,
                               tts: &[ast::TokenTree])
                               -> Box<base::MacResult+'cx> {

    match parse_args(ecx, sp, tts) {
        Some((writer, efmt, args, order, names)) => {
            MacEager::expr(expand_preparsed_format_args(ecx, sp, writer, efmt, args,
                                                        order, names))
        }
        None => DummyResult::expr(sp)
    }
}

/// Take the various parts of `format_args!(efmt, args..., name=names...)`
/// and construct the appropriate formatting expression.
pub fn expand_preparsed_format_args(ecx: &mut ExtCtxt, sp: Span,
                                    writer: P<ast::Expr>,
                                    efmt: P<ast::Expr>,
                                    args: Vec<P<ast::Expr>>,
                                    name_ordering: Vec<String>,
                                    names: HashMap<String, P<ast::Expr>>)
                                    -> P<ast::Expr> {
    let arg_types: Vec<_> = (0..args.len()).map(|_| None).collect();
    // Expand the format literal so that efmt.span will have a backtrace. This
    // is essential for locating a bug when the format literal is generated in
    // a macro. (e.g. println!("{}"), which uses concat!($fmt, "\n")).
    let efmt = ecx.expander().fold_expr(efmt);
    let mut cx = Context {
        ecx: ecx,
        writer: writer,
        args: args,
        arg_types: arg_types,
        names: names,
        name_positions: HashMap::new(),
        name_types: HashMap::new(),
        name_ordering: name_ordering,
        nest_level: 0,
        next_arg: 0,
        literal: String::new(),
        pieces: Vec::new(),
        fmtsp: efmt.span,
    };
    let fmt = match expr_to_string(cx.ecx,
                                   efmt,
                                   "format argument must be a string literal.") {
        Some((fmt, _)) => fmt,
        None => return DummyResult::raw_expr(sp)
    };

    let mut parser = parse::Parser::new(&fmt);

    while let Some(piece) = parser.next() {
        if parser.errors.len() > 0 { break }
        cx.verify_piece(&piece);
        if let Some(piece) = cx.trans_piece(&piece) {
            if !cx.literal.is_empty() {
                let s = cx.trans_literal_string();
                cx.pieces.push(s);
            }
            cx.pieces.push(piece);
        }
    }

    if !parser.errors.is_empty() {
        cx.ecx.span_err(cx.fmtsp, &format!("invalid format string: {}",
                                           parser.errors.remove(0)));
        return DummyResult::raw_expr(sp);
    }

    if !cx.literal.is_empty() {
        let s = cx.trans_literal_string();
        cx.pieces.push(s);
    }

    // Make sure that all arguments were used and all arguments have types.
    for (i, ty) in cx.arg_types.iter().enumerate() {
        if ty.is_none() {
            cx.ecx.span_err(cx.args[i].span, "argument never used");
        }
    }
    for (name, e) in &cx.names {
        if !cx.name_types.contains_key(name) {
            cx.ecx.span_err(e.span, "named argument never used");
        }
    }

    cx.into_expr()
}
