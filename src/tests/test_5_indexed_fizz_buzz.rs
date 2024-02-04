use crate::problems::_5_indexed_fizz_buzz::fizz_buzz;

#[test]
fn test_indexed_fizz_buzz() {
    assert_eq!(fizz_buzz(3), vec!["1", "2", "Fizz"]);
    assert_eq!(fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    assert_eq!(
        fizz_buzz(15),
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
    );
}
