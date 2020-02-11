pub trait AndThenSync<T, E>: futures::Future<Item = T, Error = E> + Sized {
    fn and_then_sync<U, F>(self, func: F) -> futures::future::AndThen<
        Self,
        futures::future::FutureResult<U, E>,
        Box<dyn FnOnce(T) -> futures::future::FutureResult<U, E>>,
    >
    where
        F: FnOnce(T) -> Result<U, E> + 'static,
    {
        self.and_then(Box::new(|val| futures::future::result(func(val))))
    }
}
impl<C, T, E> AndThenSync<T, E> for C where C: futures::Future<Item = T, Error = E> + Sized {}
