#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::info_print(format!($($arg)*));
    }};
}

pub fn info_print(message: String) {
    println!("\x1b[1;32minfo\x1b[0m: {}", message);
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
    eprintln!("\x1b[1;31merror\x1b[0m: {}", message);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ansi() {
        info!("hello");
        error!("world");
    }
}
