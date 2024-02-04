use crate::problems::_3_is_palindrome::is_palindrome;

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome(String::from("")), true);
    assert_eq!(is_palindrome(String::from("a")), true);
    assert_eq!(is_palindrome(String::from("Hello World")), false);
    assert_eq!(is_palindrome(String::from("aa")), true);
    assert_eq!(is_palindrome(String::from("ab")), false);
    assert_eq!(is_palindrome(String::from("0a")), false);
    assert_eq!(is_palindrome(String::from("-P")), true);
    assert_eq!(is_palindrome(String::from("aAa")), true);
    assert_eq!(
        is_palindrome(String::from("Was it a car or a cat I saw")),
        true
    );
    assert_eq!(is_palindrome(String::from("Palindrome")), false);
    assert_eq!(
        is_palindrome(String::from("Able was I ere I saw Elba")),
        true
    );
}
