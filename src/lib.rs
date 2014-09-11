#![feature(macro_rules)]

use std::io;

#[macro_export]
macro_rules! printerrln(
    ($s:expr) => (
        match io::stderr().write(format!("{:s}\n", $s).as_bytes()) {
            Ok(_) => { }
            Err(_) => { }
        }
    );

    ($pat:expr, $($itm:expr),+) => (printerrln!(format!($pat, $($itm)+ )));
)
