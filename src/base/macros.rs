#[macro_export]
macro_rules! rv {
    ($x:expr) => {
        if $x < 0 {
            Err(::core::errno::Errno(-$x as ::core::cty::c_int))
        } else {
            Ok(())
        }
    };
    ($x:expr, -> $t:ty) => {
        if $x < 0 {
            Err(::core::errno::Errno(-$x as ::core::cty::c_int))
        } else {
            Ok($x as $t)
        }
    };
}
