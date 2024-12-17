use std::sync::Arc;
use crate::common::order::Order;
use crate::common::error::Result;
use crate::common::storage::Storage;

trait OrderProcessor {
    fn process_order(&self, order: &Order) -> Result<&Order>;
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
    fn process_order(&self, order: &Order) -> Result<&Order> {
        let item_quantity = order.
    }
}