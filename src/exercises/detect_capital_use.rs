pub fn detect_capital_use(word: String) -> bool {
    assert!(word.len() > 0);
    let mut up_count = 0;
    let mut low_count = 0;
    let mut chars = word.chars();
    while let Some(char) = chars.next() {
        let up = char.is_uppercase();
        if up {
            // if has lowercase before an upper
            if low_count > 0 {
                return false;
            }
            up_count += 1;
        } else {
            if up_count > 1 {
                // not just starting from uppercase
                return false;
            }
            low_count += 1;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cap() {
        assert_eq!(detect_capital_use("USA".to_string()), true);
        assert_eq!(detect_capital_use("Flag".to_string()), true);
        assert_eq!(detect_capital_use("flag".to_string()), true);
        assert_eq!(detect_capital_use("flaG".to_string()), false);
        assert_eq!(detect_capital_use("FlaG".to_string()), false);
        assert_eq!(detect_capital_use("FLag".to_string()), false);
    }
}