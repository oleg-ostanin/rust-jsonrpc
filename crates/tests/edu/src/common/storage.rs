use std::collections::HashMap;

pub(crate) struct Storage {
    items: HashMap<u32, u32>,
}

impl Storage {
    pub fn new() -> Self {
        Self { items: HashMap::new() }
    }

    // pub fn items(&self) -> &HashMap<u32, u32> {
    //     &self.items
    // }

    pub fn items(&mut self) -> &mut HashMap<u32, u32> {
        &mut self.items
    }
}