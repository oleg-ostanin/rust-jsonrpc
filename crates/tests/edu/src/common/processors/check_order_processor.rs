use std::sync::Arc;
use crate::common::order::Order;
use crate::common::error::{Result, Error};
use crate::common::processors::order_processor::OrderProcessor;
use crate::common::storage::Storage;

struct CheckStorageOrderProcessor {
    storage: Arc<Storage>,
}

impl CheckStorageOrderProcessor {
    pub fn new(storage: Arc<Storage>) -> Self {
        Self { storage }
    }
}

impl OrderProcessor for CheckStorageOrderProcessor {
    fn process_order(&self, order: Order) -> Result<Order> {
        let order_items = order.items();
        let storage_items = self.storage.items();
        for (item_id, order_quantity) in order_items.iter() {
            let storage_quantity = storage_items.get(&item_id).ok_or(Error::NotEnoughItems)?;
            if storage_quantity < order_quantity {
                return Err(Error::NotEnoughItems);
            }
        }
        Ok(order)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use crate::common::error::Error;
    use crate::common::order::Order;
    use crate::common::processors::check_order_processor::CheckStorageOrderProcessor;
    use crate::common::processors::order_processor::OrderProcessor;
    use crate::common::storage::Storage;

    fn check_storage() -> CheckStorageOrderProcessor {
        let storage = Arc::new(Storage::default());
        CheckStorageOrderProcessor::new(storage)
    }

    #[test]
    fn enough() {
        let mut items = HashMap::new();
        items.insert(2u32, 4);
        let order = Order::default_with_items(items);
        let res = check_storage().process_order(order);
        assert!(res.is_ok())
    }

    #[test]
    fn not_enough() {
        let mut items = HashMap::new();
        items.insert(2u32, 5);
        let order = Order::default_with_items(items);
        let res = check_storage().process_order(order);
        assert!(res.is_err_and(|e| e == Error::NotEnoughItems ))
    }
}