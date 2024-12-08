#[macro_export]
macro_rules! all_true {
    ( $( $x:expr ),* ) => {
        {
            let mut res = true;
            $(
                if(!$x) {
                    res = false;
                }
            )*
            res
        }
    };
}

fn always_true() -> bool {
    true
}

fn equals(x: i32, y: i32) -> bool {
    x == y
}

#[cfg(test)]
mod tests {
    use crate::macros::hw::{always_true, equals};

    #[test]
    fn check_macro() {
        let res = all_true!(always_true(), equals(2, 2), equals(3, 3), equals(3, 3));
        println!("res: {:?}", res);
    }
}