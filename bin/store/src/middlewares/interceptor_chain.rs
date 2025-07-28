use tonic::service::Interceptor;
use tonic::{Request, Status};

/// A chain of two interceptors that applies them in sequence
#[derive(Clone)]
pub struct InterceptorChain<A, B> {
    first: A,
    second: B,
}

impl<A, B> InterceptorChain<A, B> {
    pub fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}

impl<A, B> Interceptor for InterceptorChain<A, B>
where
    A: Interceptor + Clone,
    B: Interceptor + Clone,
{
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // Apply the first interceptor
        let mut first = self.first.clone();
        let request = first.call(request)?;

        // Apply the second interceptor
        let mut second = self.second.clone();
        second.call(request)
    }
}
