use std::ops::{Index, Range};

use num_bigint::BigInt;


pub struct OpStack {
    /// The underlying, actual stack. When manually accessing, be aware of reversed indexing:
    /// while `op_stack[0]` is the top of the stack, `op_stack.stack[0]` is the lowest element in
    /// the stack.
    pub stack: Vec<BigInt>,
    //_phantom_data: std::marker::PhantomData<&'a ()>
}


impl Index<Range<usize>> for OpStack {
    type Output = [BigInt];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        if range.end <= self.stack.len() && range.start < range.end {
            &self.stack[range]
        } else {
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex, OnceLock};
    struct Global(u32);
    static CELL: OnceLock<Arc<Mutex<Global>>> = OnceLock::new();

    #[test]
    fn once() {
        // `OnceLock` has not been written to yet.
        assert!(CELL.get().is_none());

        // Spawn a thread and write to `OnceLock`.
        std::thread::spawn(|| {
            let value = CELL.get_or_init(|| Arc::new(Mutex::new(Global(42))));
        }).join().unwrap();
    }
}