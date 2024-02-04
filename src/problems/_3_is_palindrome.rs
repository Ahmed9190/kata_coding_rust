pub trait StringX {
    fn remove_spaces(&self) -> String;

    fn to_alphanumeric_characters(&self) -> Vec<char>;
}

impl StringX for String {
    fn remove_spaces(&self) -> String {
        self.replace(' ', "")
    }

    fn to_alphanumeric_characters(&self) -> Vec<char> {
        self.chars().filter(|&c| c.is_alphanumeric()).collect()
    }
}

pub fn is_palindrome(text: String) -> bool {
    let formatted_text = text
        .remove_spaces()
        .to_ascii_lowercase()
        .to_alphanumeric_characters();

    let formatted_text_length = formatted_text.len();

    if formatted_text_length <= 1 {
        return true;
    }
    let mut p1: usize = 0;
    let mut p2 = formatted_text_length - 1;

    while p1 < p2 {
        if formatted_text[p1] != formatted_text[p2] {
            return false;
        }
        p1 += 1;
        p2 -= 1;
    }

    true
}
