pub struct OffsetConstructor<T> {
    pub request_body: T,
}

impl<T> OffsetConstructor<T>
where
    T: OffsetQueryFilter,
{
    pub fn new(request_body: T) -> Self {
        Self { request_body }
    }

    /// Constructs the OFFSET clause for SQL queries
    pub fn construct_offset(&self) -> String {
        if self.request_body.get_offset() > 0 {
            format!(" OFFSET {}", self.request_body.get_offset())
        } else {
            String::from("")
        }
    }
}

/// Trait defining the required methods for OFFSET construction
pub trait OffsetQueryFilter {
    fn get_offset(&self) -> usize;
}
