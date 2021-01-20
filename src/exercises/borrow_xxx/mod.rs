#[derive(Debug)]
struct A {
    x: String,
    y: String,
}

impl A {
    fn get_a_mut(self: &mut A) -> &'_ mut String {
        &mut self.x
    }
}

trait XXX {
    fn xxx<F: Fn() -> String>(self: &mut Self, s: F);
}

impl XXX for String {
    fn xxx<F>(self: &mut String, s: F)
        where F: Fn() -> String
    {
        self.push_str(&s());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_test() {
        let mut a: A = A { x: String::from("x"), y: String::from("y") };

        println!("{:?}", a);

        let y = &a.y;
        a.get_a_mut().xxx(|| String::from(y));
        println!("{:?}", a);
    }
}
