#[cfg(feature = "print-stdout")]
pub use libc_print::libc_println as println;
#[cfg(not(feature = "print-stdout"))]
#[macro_export]
macro_rules! println {
    ( $( $x:expr ),* ) => {{ /* Do nothing */ }};
}
