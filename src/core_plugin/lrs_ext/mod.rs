pub use self::to::{
    derive_to, derive_try_to, derive_to_for_copy, derive_copy_to_for,
};
pub use self::eq::{derive_eq};
pub use self::copy::{derive_copy, derive_copy_and_to, derive_pod_copy_and_to};
pub use self::format::{expand_format_args};
pub use self::debug::{derive_debug};

macro_rules! pathvec {
    ($($x:ident)::+) => (
        vec![ $( stringify!($x) ),+ ]
    )
}

macro_rules! path {
    ($($x:tt)*) => (
        ::syntax_ext::deriving::generic::ty::Path::new( pathvec!( $($x)* ) )
    )
}

macro_rules! path_local {
    ($x:ident) => (
        ::syntax_ext::deriving::generic::ty::Path::new_local(stringify!($x))
    )
}

macro_rules! pathvec_std {
    ($cx:expr, $first:ident :: $($rest:ident)::+) => ({
        let mut v = pathvec!($($rest)::+);
        if let Some(s) = $cx.crate_root {
            v.insert(0, s);
        }
        v
    })
}

macro_rules! path_std {
    ($($x:tt)*) => (
        ::syntax_ext::deriving::generic::ty::Path::new( pathvec_std!( $($x)* ) )
    )
}

mod to;
mod eq;
mod copy;
mod format;
mod debug;
