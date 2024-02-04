use crate::problems::_2_reverse_words::reverse_word;

#[test]
fn test_reverse_words() {
    assert_eq!(
        reverse_word("Rust programming language"),
        "language programming Rust"
    );
    assert_eq!(reverse_word("123 456 789"), "789 456 123");
    assert_eq!(reverse_word("SingleWord"), "SingleWord");
    assert_eq!(reverse_word(""), "");
}
