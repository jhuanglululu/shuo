# shuo (說)

A tiny logging crate for Rust that just says things. `shuo` provides two simple macros — `info!` and `error!` — for printing colored, prefixed messages to stdout and stderr, plus an `error_out!` macro for printing an error and exiting in one step. It also offers `ok_or_error_out!` and `some_or_error_out!` for unwrapping a `Result` or `Option`, exiting with an error message instead of panicking. No log levels, no filters, no configuration: just `shuo::info!("hello")` and you're done.

## Usage

```rust
use shuo::{info, error, error_out, ok_or_error_out, some_or_error_out};

info!("starting up on port {}", 8080);
error!("something went wrong: {}", err);
error_out!(1, "fatal: {}", err);
let config = ok_or_error_out!(load_config(path), 1);
let name = some_or_error_out!(config.name, 1, "no name set");
```
