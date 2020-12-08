pub fn test_pig_latin() {
    println!("first -> {}", word_to_pig_latin("first"));
    println!("apple -> {}", word_to_pig_latin("apple"));
}

pub fn word_to_pig_latin(string: &str) -> String {
    if string.len() < 2 {
        return String::from(string);
    }
    let prefix_char = string.chars().nth(0).unwrap();
    return if let 'a' | 'e' | 'i' | 'o' | 'u' = string.chars().nth(0).unwrap() {
        let mut result = String::from(string);
        result.push('-');
        result.push_str("hay");
        result
    } else {
        let mut result = String::from(&string[1..]);
        result.push('-');
        result.push(prefix_char);
        result.push_str("ay");
        result
    }
