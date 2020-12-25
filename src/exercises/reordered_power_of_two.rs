pub fn reordered_power_of2(n: i32) -> bool {
    let mut digits: Vec<i32> = extract_digits(n);
    digits.sort();

    if digits[0] != 0 && is_power_of2(to_number(&digits)) {
        return true;
    }
    while next_permutation(&mut digits) {
        if digits[0] == 0 {
            continue;
        }
        if is_power_of2(to_number(&digits)) {
            return true;
        }
    }
    return false;
}

fn next_permutation(array: &mut Vec<i32>) -> bool {
    let first = 0;
    let last = array.len();

    if first == last {
        return false;
    }

    let mut i = first;
    i += 1;
    if i == last {
        return false;
    }
    i = last;
    i -= 1;

    loop {
        let ii = i;
        i -= 1;
        if array[i] < array[ii] {
            let mut j = last - 1;
            while !(array[i] < array[j]) {
                j -= 1;
            }
            array.swap(i, j);
            array[ii..].reverse();
            //std::reverse(ii, last);
            return true;
        }
        if i == first {
            array.reverse();
            // std::reverse(first, last);
            return false;
        }
    }
}

fn extract_digits(mut n: i32) -> Vec<i32> {
    let mut digits = Vec::new();

    while n > 0 {
        let rem = n % 10;
        digits.push(rem);
        n = (n - rem) / 10;
    }

    digits.reverse();
    digits
}

fn to_number(digits: &[i32]) -> i32 {
    let mut n = 0;
    for digit in digits {
        n = n * 10 + digit;
    }
    n
}

fn is_power_of2(mut n: i32) -> bool {
    while n > 0 && (n & 1) == 0 {
        n = n >> 1;
    }

    n == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_number() {
        assert_eq!(to_number(&[3, 2, 1]), 321);
        assert_eq!(to_number(&[1, 2, 0]), 120);
        assert_eq!(to_number(&[]), 0);
        assert_eq!(to_number(&[0, 1]), 1);
        assert_eq!(to_number(&[1, 0]), 10);
        assert_eq!(to_number(&[1, 0, 1]), 101);
    }

    #[test]
    fn test_extract_digits() {
        assert_eq!(extract_digits(123), &[1, 2, 3]);
        assert_eq!(extract_digits(120), &[1, 2, 0]);
        assert_eq!(extract_digits(0), &[]);
        assert_eq!(extract_digits(10), &[1, 0]);
        assert_eq!(extract_digits(100), &[1, 0, 0]);
    }

    #[test]
    fn test_is_power_of2() {
        assert_eq!(is_power_of2(123), false);
        assert_eq!(is_power_of2(0), false);
        assert_eq!(is_power_of2(20000), false);
        let mut n = 1;
        for _ in 1..28 {
            assert_eq!(is_power_of2(n), true);
            n *= 2;
        }
    }

    #[test]
    fn test_reordered_power_of2() {
        assert_eq!(reordered_power_of2(1), true);
        assert_eq!(reordered_power_of2(10), false);
        assert_eq!(reordered_power_of2(16), true);
        assert_eq!(reordered_power_of2(24), false);
        assert_eq!(reordered_power_of2(46), true);
    }
}