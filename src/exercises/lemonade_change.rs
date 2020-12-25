use std::cmp::Reverse;
use std::collections::BTreeMap;

const LEMONADE_PRICE: i32 = 5;

pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut change_bank = BTreeMap::new();
    for bill in bills {

        *change_bank.entry(Reverse(bill)).or_insert(0) += 1;

        if bill != LEMONADE_PRICE {
            let mut change_left = bill - LEMONADE_PRICE;
            for (&Reverse(c), n) in change_bank.iter_mut() {
                while *n > 0 && change_left >= c {
                    *n -= 1;
                    change_left -= c;
                }
                if change_left <= 0 {
                    break;
                }
            }

            if change_left != 0 {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lemonade_change() {
        assert_eq!(lemonade_change(vec![5, 5, 5, 10, 20]), true);
        assert_eq!(lemonade_change(vec![5, 5, 10]), true);
        assert_eq!(lemonade_change(vec![10, 10]), false);
        assert_eq!(lemonade_change(vec![5, 5, 10, 10, 20]), false);
    }
}