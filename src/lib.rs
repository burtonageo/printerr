#![feature(macro_rules)]

#[macro_export]
macro_rules! err_print(
    ($msg:expr) => (
        match io::stderr().write(format!("{:s}", $msg).as_bytes()) {
            Ok(_) => { }
            Err(_) => { }
        }
    );

    ($fmt:expr, $($xs:expr)*) => (printerrln!(format!($fmt, $($xs)* )));
)

#[macro_export]
macro_rules! err_println(
    ($msg:expr) => (printerr!(format!("{:s}\n", $msg)));
    ($fmt:expr, $($xs:expr)*) => (printerrln!(format!($fmt, $($xs)* )));
)
