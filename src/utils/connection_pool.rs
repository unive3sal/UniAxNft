use std::sync::{
    Arc,
    Mutex
};

pub struct ConnectionPool<T> {
    connections: Vec<T>,
    current_idx: Arc<Mutex<usize>>,
}

impl<T> ConnectionPool<T> {
    /* TODO:
    pub fn new(pool_size: usize) -> Self {
        Self {
        }
    }
    */
}
