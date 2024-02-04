// Problem: FizzBuzz
// Write a function that takes a number as input and returns a string based on the following rules:

// If the number is divisible by 3, return "Fizz".
// If the number is divisible by 5, return "Buzz".
// If the number is divisible by both 3 and 5, return "FizzBuzz".
// Otherwise, return the number as a string.
pub fn fizz_buzz(number: i32) -> String {
    let mut string_buffer = String::new();

    if number % 3 == 0 {
        string_buffer.push_str("Fizz");
    }

    if number % 5 == 0 {
        string_buffer.push_str("Buzz");
    }

    if string_buffer.is_empty() {
        string_buffer.push_str(number.to_string().as_str());
    }

    string_buffer
}
