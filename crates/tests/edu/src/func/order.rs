use std::collections::HashMap;
use crate::common::order::Order;
use crate::common::storage::Storage;

fn default_storage(num: u32) -> Storage {
    let mut storage = Storage::new();

    let mut items = storage.items();
    for i in 0..num {
        items.insert(i, i*2);
    }
    storage
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple() {

    }
}