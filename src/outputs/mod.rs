mod console;
pub use console::write as console;
pub mod discord;

#[macro_export]
macro_rules! write {
    ($($arg:tt)*) => {
        async {
            use std::fmt;
            use $crate::{outputs::*, write_to};

            let mut string = String::new();
            let _ = fmt::write(&mut string, format_args!($($arg)*));
            write_to!(&string, console, discord::webhook).await;
        }
    };
}

#[macro_export]
macro_rules! writeln {
    ($($arg:tt)*) => {
        async {
            use std::fmt;
            use $crate::{outputs::*, write_to};

            let mut string = String::new();
            let _ = fmt::write(&mut string, format_args_nl!($($arg)*));
            write_to!(&string, console, discord::webhook).await;
        }
    };
}

#[macro_export]
macro_rules! write_to {
    ($arg:expr, $($output:path),*) => {
        async {
            $(
                let _ = $output($arg).await;
            )*
        }
    };
}
