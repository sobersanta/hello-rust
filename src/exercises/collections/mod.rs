// Given a list of integers, use a vector and return the
// mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
use std::collections::HashMap;

pub fn mode(values: &Vec<i64>) -> Option<i64> {
    if values.len() == 0 {
        return None;
    }
    let mut frequencies: HashMap<i64, u16> = HashMap::new();
    let mut max_cnt = 0_u16;
    let mut max = 0_i64;
    for value in values {
        let f = frequencies.entry(*value).or_insert(0);
        *f += 1;
        if *f > max_cnt {
            max = *value;
            max_cnt = *f;
        }
    }
    return Some(max);
}

pub fn median(values: &Vec<i64>) -> Option<i64> {
    if values.len() == 0 {
        return None;
    }
    let mut sorted_values = values.clone();
    sorted_values.sort();
    return Some(sorted_values[sorted_values.len() / 2]);
}

pub fn mean_dumb(values: &Vec<i64>) -> Option<f64> {
    if values.len() == 0 {
        return None;
    }
    let mut sum = 0;
    for value in values {
        sum += value;
    }
    return Some(sum as f64 / (values.len() as f64));
}

pub fn mean_incremental(values: &Vec<i64>) -> Option<f64> {
    if values.len() == 0 {
        return None;
    }
    let mut mean = 0.0_f64;
    let mut n = 1.0_f64;
    for value in values {
        mean = mean + (*value as f64 - mean) / n;
        n += 1.0_f64;
    }
    return Some(mean);
}

#[cfg(tests)]
mod tests;