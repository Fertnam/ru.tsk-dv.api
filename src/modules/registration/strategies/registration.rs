pub trait RegistrationStrategy<T, U> {
    fn register(&self, dto: &T) -> U;
}