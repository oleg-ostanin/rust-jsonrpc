use std::sync::Arc;
use crate::common::order::Order;
use crate::common::error::{Result, Error};
use crate::common::storage::Storage;

trait OrderProcessor {
    fn process_order(&self, order: Order) -> Result<Order>;
}

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
        for (item_id, order_quantity) in order_items.iter() {
            let storage_quantity = self.storage.items().get(&item_id).ok_or(Error::NotEnoughItems)?;
            if storage_quantity < order_quantity {
                return Err(Error::NotEnoughItems);
            }
        }
        Ok(order)
    }
}