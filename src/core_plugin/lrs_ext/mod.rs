pub use self::clone::{derive_clone, derive_clone_for_copy, derive_copy_clone_for};
pub use self::eq::{derive_eq};
pub use self::copy::{derive_copy, derive_copy_and_clone};
pub use self::format::{expand_format_args};

macro_rules! pathvec {
    ($($x:ident)::+) => (
        vec![ $( stringify!($x) ),+ ]
    )
}

macro_rules! path {
    ($($x:tt)*) => (
        ::ext::deriving::generic::ty::Path::new( pathvec!( $($x)* ) )
    )
}

macro_rules! path_local {
    ($x:ident) => (
        ::ext::deriving::generic::ty::Path::new_local(stringify!($x))
    )
}

macro_rules! pathvec_std {
    ($cx:expr, $first:ident :: $($rest:ident)::+) => (
        if $cx.use_std {
            pathvec!(std :: $($rest)::+)
        } else {
            pathvec!($first :: $($rest)::+)
        }
    )
}

macro_rules! path_std {
    ($($x:tt)*) => (
        ::ext::deriving::generic::ty::Path::new( pathvec_std!( $($x)* ) )
    )
}

mod clone;
mod eq;
mod copy;
mod format;
