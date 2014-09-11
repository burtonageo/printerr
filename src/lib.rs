#![feature(macro_rules)]

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
    ($msg:expr) => (printerr!(format!("{}\n", $msg)));
    ($fmt:expr, $($xs:expr)*) => (printerrln!(format!($fmt, $($xs)* )));
)
