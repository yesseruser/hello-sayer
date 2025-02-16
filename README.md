# hello-sayer

A Rust library for generating and printing Hello world messages.

## Usage
Install the library with:
```bash
cargo add hello-sayer
```

The following functions are included:
- `generate_hello_message() -> &'static str` - Returns `"Hello world!"` as a string slice
- `say_hello() -> ()` - Prints the string returned by `generate_hello_message`
- `say_hello_string(&str string) -> ()` - Prints the string slice given.
- `say_hello_handler(fn() -> &'static str) -> ()` - Prints the string slice returned by the given handler.
