use procedural_macro::HelloMacro;

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        Pancakes::hello_macro();
    }
}