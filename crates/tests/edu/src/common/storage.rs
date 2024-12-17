use std::collections::HashMap;

pub(crate) struct Storage {
    items: HashMap<u32, u32>,
}

impl Storage {
    pub fn new() -> Self {
        Self { items: HashMap::new() }
    }

    pub fn items(&self) -> &HashMap<u32, u32> {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut HashMap<u32, u32> {
        &mut self.items
    }
}

impl Default for Storage {
    fn default() -> Self {
        let mut storage = Storage::new();
        let mut items = storage.items_mut();
        for i in 0..7 {
            items.insert(i, i*2);
        }
        storage
    }
}