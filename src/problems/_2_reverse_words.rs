pub fn reverse_word(text: &str) -> String {
    text.split(' ')
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
        .to_string()
}
