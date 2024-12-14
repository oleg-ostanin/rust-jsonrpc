use std::collections::HashMap;
use crate::iter::common::Types;

fn default_map(size: usize) -> HashMap<i32, Types> {
    let mut res = HashMap::new();
    for i in 0..size {
        res.insert(i as i32, Types::new(i as i32));
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::iter::map::default_map;

    #[test]
    // iter does not consume values
    fn iter() {
        let map = default_map(7);
        map.iter().for_each(
            // t is a reference
            |t| println!("{:?}", t)
        );
        assert_eq!(7, map.len());
    }
}