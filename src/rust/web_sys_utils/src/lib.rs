#[macro_export]
macro_rules! debug {
    (@ $lit:literal $(, $arg:expr )* ) => {
        ::web_sys::console::log_1(&format!("{}:{}: {}", file!(), line!(), format!($lit $(, $arg )*)).into());
    };
    ($lit:literal $(, $arg:expr )* ) => {
        ::web_sys::console::log_1(&format!($lit $(, $arg )*).into());
    };
}

#[macro_export]
macro_rules! warn {
    (@ $lit:literal $(, $arg:expr )* ) => {
        ::web_sys::console::warn_1(&format!("{}:{}: {}", file!(), line!(), format!($lit $(, $arg )*)).into());
    };
    ($lit:literal $(, $arg:expr )* ) => {
        ::web_sys::console::warn_1(&format!($lit $(, $arg )*).into());
    };
}

#[macro_export]
macro_rules! error {
    (@ $lit:literal $(, $arg:expr )* ) => {
        ::web_sys::console::error_1(&format!("{}:{}: {}", file!(), line!(), format!($lit $(, $arg )*)).into());
    };
    ($lit:literal $(, $arg:expr )* ) => {
        ::web_sys::console::error_1(&format!($lit $(, $arg )*).into());
    };
}
