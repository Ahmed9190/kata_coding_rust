/*
https://leetcode.com/problems/fizz-buzz/description/
Given an integer n, return a string array answer (1-indexed) where:

answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
answer[i] == "Fizz" if i is divisible by 3.
answer[i] == "Buzz" if i is divisible by 5.
answer[i] == i (as a string) if none of the above conditions are true.

Example 1:

Input: n = 3
Output: ["1","2","Fizz"]
Example 2:

Input: n = 5
Output: ["1","2","Fizz","4","Buzz"]
Example 3:

Input: n = 15
Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]

Constraints:

1 <= n <= 104
*/
pub fn fizz_buzz(number: i32) -> Vec<String> {
    let mut answer: Vec<String> = vec![];

    for i in 1..=number {
        answer.push(fizz_buzz_decider(i));
    }

    answer
}

fn fizz_buzz_decider(number: i32) -> String {
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
