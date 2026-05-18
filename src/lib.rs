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
    ($($arg:tt)+) => {{
    $crate::error_print(format!($($arg)+));
    }};
}

#[macro_export]
macro_rules! error_out {
	($exit:expr, $($arg:tt)+) => {{
		$crate::error_print(format!($($arg)+));
        std::process::exit($exit);
    }};
}

#[macro_export]
macro_rules! ok_or_error_out {
    ($expr:expr, $code:expr $(,)?) => {
        match $expr {
            Ok(v) => v,
            Err(e) => $crate::error_out!($code, "{}", e),
        }
    };
}

#[macro_export]
macro_rules! some_or_error_out {
	($expr:expr, $code:expr, $($arg:tt)+) => {
        match $expr {
            Some(v) => v,
            None => $crate::error_out!($code, $($arg)+),
        }
    };
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

    #[test]
    fn test_macros_expand() {
        let _ = ok_or_error_out!(Ok::<i32, String>(1), 1);
        let _ = some_or_error_out!(Some(1), 1, "unreachable");
    }
}
