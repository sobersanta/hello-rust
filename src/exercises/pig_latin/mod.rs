// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

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
    };
}

#[cfg(test)]
mod tests;