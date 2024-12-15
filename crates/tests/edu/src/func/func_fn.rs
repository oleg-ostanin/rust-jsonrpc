struct AddCalc<T: Fn(i32, i32) -> i32> {
    pub add_calc: T,
}

impl<T: Fn(i32, i32) -> i32> AddCalc<T> {
    pub fn new(add_calc: T) -> Self {
        Self { add_calc }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let add: fn(i32, i32) -> i32 = |a, b| a + b;
        let sum = add(3, 5);
        assert_eq!(8, sum)
    }

    #[test]
    fn add_struct() {
        let add: fn(i32, i32) -> i32 = |a, b| a + b;
        let add_instance = AddCalc::new(add);
        // does not compile!
        // let sum = add_instance.add_calc(3, 5);
        let func = add_instance.add_calc;
        let sum = func(3, 5);
        assert_eq!(8, sum)
    }
}