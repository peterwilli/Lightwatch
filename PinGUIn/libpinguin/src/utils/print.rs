#[cfg(feature = "print-stdout")]
{
    pub use libc_print::libc_println;
#[macro_export]
macro_rules! println {
    ( $( $x:expr ),* ) => {{
        libc_println!($x)
    }};
}
}
#[cfg(not(feature = "print-stdout"))]
#[macro_export]
macro_rules! println {
    ( $( $x:expr ),* ) => {{ /* Do nothing */ }};
}
