pub trait RegistrationStrategy<T> {
    fn register(&self, dto: &T);
}