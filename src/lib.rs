#![feature(macro_rules)]

#[macro_export]
macro_rules! err_print(
    ($msg:expr) => (
        match io::stderr().write(format!("{:s}", $msg).as_bytes()) {
            Ok(_) => { }
            Err(_) => { }
        }
    );

    ($fmt:expr, $($xs:expr)*) => (err_print!(format!($fmt, $($xs)* )));
)

#[macro_export]
macro_rules! err_println(
    ($msg:expr) => (err_print!(format!("{:s}\n", $msg)));
    ($fmt:expr, $($xs:expr)*) => (err_println!(format!($fmt, $($xs)* )));
)
