use errno::{Errno};

pub type Result<T> = ::std::result::Result<T, Errno>;
