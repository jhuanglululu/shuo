use colored::Colorize;

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::info_print(format!($($arg)*));
    }};
}

pub fn info_print(message: String) {
    println!("{:}: {}", "info".green().bold(), message);
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::error_print(format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error_out {
    ($exit:expr, $($arg:tt)*) => {{
        $crate::error_print(format!($($arg)*));
        std::process::exit($exit);
    }};
}

pub fn error_print(message: String) {
    eprintln!("{:}: {}", "error".red().bold(), message);
}
