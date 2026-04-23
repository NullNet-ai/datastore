use hlc::Timestamp;

#[derive(Clone, Debug)]
pub struct MutableTimestamp {
    inner: Timestamp,
}
#[allow(warnings)]
impl MutableTimestamp {
    pub fn new(physical: u64, logical: u64, node_id: String) -> Self {
        Self {
            inner: Timestamp::new(logical, physical, node_id),
        }
    }

    pub fn from(timestamp: &Timestamp) -> Self {
        Self {
            inner: Timestamp::new(
                timestamp.logical,
                timestamp.physical,
                timestamp.node_id.clone(),
            ),
        }
    }

    pub fn set_physical(&mut self, n: u64) {
        self.inner = Timestamp::new(self.inner.logical, n, self.inner.node_id.clone());
    }

    pub fn set_logical(&mut self, n: u64) {
        self.inner = Timestamp::new(n, self.inner.physical, self.inner.node_id.clone());
    }

    pub fn set_node_id(&mut self, n: String) {
        self.inner = Timestamp::new(self.inner.logical, self.inner.physical, n);
    }
}

// Implement Deref to allow using Timestamp methods directly
impl std::ops::Deref for MutableTimestamp {
    type Target = Timestamp;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
