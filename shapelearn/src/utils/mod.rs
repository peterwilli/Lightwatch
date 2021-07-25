#[cfg(feature = "print-stdout")]
mod print {
    #[macro_export]
    macro_rules! println {
        ( $( $x:expr ),* ) => {{
            use libc_print::libc_println;
            libc_println!($($x),*);
        }};
    }
}

#[cfg(not(feature = "print-stdout"))]
mod print {
    #[macro_export]
    macro_rules! println {
        ( $( $x:expr ),* ) => {{ /* doing nothing */ }};
    }
}
