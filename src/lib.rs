#![feature(macro_rules)]

use std::io;

#[macro_export]
macro_rules! printerr(
    ($msg:expr) => (
        match io::stderr().write(format!("{:s}", $msg).as_bytes()) {
            Ok(_) => { }
            Err(_) => { }
        }
    );

    ($fmt:expr, $($xs:expr)*) => (printerrln!(format!($fmt, $($xs)* )));
)

#[macro_export]
macro_rules! printerrln(
    ($msg:expr) => (printerr!(format!("{:s}\n", $msg).as_bytes()));
    ($fmt:expr, $($xs:expr)*) => (printerrln!(format!($fmt, $($xs)* )));
)
