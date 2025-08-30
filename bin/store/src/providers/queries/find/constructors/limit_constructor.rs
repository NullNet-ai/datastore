pub struct LimitConstructor<T> {
    pub request_body: T,
}

impl<T> LimitConstructor<T>
where
    T: LimitQueryFilter,
{
    pub fn new(request_body: T) -> Self {
        Self { request_body }
    }

    /// Constructs the LIMIT clause for SQL queries
    pub fn construct_limit(&self) -> String {
        if self.request_body.get_limit() > 0 {
            format!(" LIMIT {}", self.request_body.get_limit())
        } else {
            String::from("LIMIT 10")
        }
    }
}

/// Trait defining the required methods for LIMIT construction
pub trait LimitQueryFilter {
    fn get_limit(&self) -> usize;
}
