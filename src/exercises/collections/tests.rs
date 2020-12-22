use super::*;

#[test]
pub fn test() {
    let values: Vec<i64> = vec![1, 100, 2, 200, 3, 300, 5, 4, 6, 9, 8, 0, 9];
    // println!("Mean using dump algo: {:?}", mean_dumb(&values));
    assert_eq!(49.76923076923077, mean_dumb(&values).unwrap());
    assert_eq!(49.76923076923077, mean_incremental(&values).unwrap());
    assert_eq!(6_i64, median(&values).unwrap());
    assert_eq!(9_i64, mode(&values).unwrap());
    // println!("Mean using incremental algo: {:?}", mean_incremental(&values));
    // println!("Median: {:?}", median(&values));
    // println!("Mode: {:?}", mode(&values));
}
