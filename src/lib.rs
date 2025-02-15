pub fn generate_hello_message() -> &'static str {
    "Hello world!"
}

pub fn say_hello() {
    println!("{}", generate_hello_message())
}

pub fn say_hello_string(hello_string: &str) {
    println!("{}", hello_string)
}

pub fn say_hello_handler(hello_handler: fn() -> &'static str) {
    println!("{}", hello_handler())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_generate_hello_message_return_hello_world() {
        let result = generate_hello_message();
        assert_eq!(result, "Hello world!");
    }

    #[test]
    fn does_say_hello_not_fail() {
        say_hello()
    }

    #[test]
    fn does_say_hello_string_not_fail() {
        let string = generate_hello_message();
        say_hello_string(string)
    }

    #[test]
    fn does_say_hello_handler_not_fail() {
        say_hello_handler(generate_hello_message)
    }
}
