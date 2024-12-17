use std::collections::HashMap;

pub(crate) struct Storage {
    items: HashMap<u32, u32>,
}

impl Storage {
    pub fn new() -> Self {
        Self { items: HashMap::new() }
    }
}