use crate::problems::_1_add_two_numbers::add_two_numbers;

#[test]
fn test_add_two_numbers() {
    assert_eq!(add_two_numbers(2, 3), 5);
    assert_eq!(add_two_numbers(1, -1), 0);
}
