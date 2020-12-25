pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
    let mut accum = n;
    let mask = 2147483647;

    for i in 0..31 {
        let bit_mask = 1 << (30 - i);
        if (accum & bit_mask) != 0 {
            let tst = n & !(mask >> (i + 1));
            if tst > m {
                accum &= !bit_mask;
            }
        }
    }
    return accum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_bitwise_and() {
        assert_eq!(range_bitwise_and(5, 7), 4);
        assert_eq!(range_bitwise_and(0, 2147483647), 0);
        assert_eq!(range_bitwise_and(0, 1), 0);
    }
}