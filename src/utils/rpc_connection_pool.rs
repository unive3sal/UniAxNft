use std::sync::{
    Arc,
    Mutex
};

pub struct RpcConnectionPool<T> {
    connections: Vec<T>,
    current_idx: Arc<Mutex<usize>>,
}

impl RpcConnectionPool<T> {
    /* TODO:
    pub fn new(pool_size: usize) -> Self {
        Self {
        }
    }
    */
}
