
# Extended Rust

A set of handy macros for your Rust projects

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)


## Installation

Install Extended Rust with cargo

```toml
cargo.toml:
extended_rust = [ git = "https://github.com/AlexIndustrial/extended_rust.git" ]
```

## Usage/Examples

Create String

```rust
let my_string = string!(); // String::new()

let my_string = string!("hello world");
// "hello world".to_string()
```

Conversion of Option to Result and conversely

```rust
let option: Option<String>= Some(string!("hello world"));

let result: Result<String, ()> = to_result!(option);
let result_with_err: Result<String, i32> = to_result!(option, 10); 

let back_to_opt: Option<String> = to_option!(result);
let back_to_opt: Option<String> = to_option!(result_with_err);
```

Returns by type

```rust
some_return!(option, string!("return this if option is Some"));
none_return!(option, string!("return this if option is None"));

ok_return!(result, string!("return this if result is Ok"));
err_return!(result, string!("return this if result is Err"));
```

Quick return result

```rust 
ok!(string!("data")); // return Ok(string!("data"))
err!(string!("data")); // return Err(string!("data"))
```

Io Result

```rust
fn main() -> io_result!(()) /* std::io::Result<()> */ {
    Ok(())
}
```

Prints
```rust
q_print_obj!(obj); // println!("{:?}", obj)
q_print!("data") // println!("{}", "data")
```