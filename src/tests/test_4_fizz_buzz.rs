use crate::problems::_4_fizz_buzz::fizz_buzz;

#[test]
pub fn test_fizz_buzz() {
    assert_eq!(fizz_buzz(10), "Buzz");
    assert_eq!(fizz_buzz(30), "FizzBuzz");
    assert_eq!(fizz_buzz(7), "7");
    assert_eq!(fizz_buzz(9), "Fizz");
}
