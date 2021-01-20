use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
struct Cacher<T, V, R>
    where T: Fn(&V) -> R,
          V: Eq + Hash + Copy + Debug,
          R: Debug
{
    calculation: T,
    values: HashMap<V, R>,
}

impl<T, V, R> Cacher<T, V, R>
    where
        T: Fn(&V) -> R,
        V: Eq + Hash + Copy + Debug,
        R: Debug
{
    fn new(calculation: T) -> Cacher<T, V, R> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &V) -> &R {
        let calculation = &self.calculation;
        let x = self.values.entry(*arg).or_insert_with(move || {
            let c = &*calculation;
            c(arg)
        });
        let _ = (calculation)(arg);
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut i32_cacher = Cacher::new(|x| x + 1);
        println!("{}", i32_cacher.value(&25));
        println!("{}", i32_cacher.value(&25));
        println!("{}", i32_cacher.value(&1));
        println!("{}", i32_cacher.value(&1));
    }
}
