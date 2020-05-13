pub trait DbClient<T> {
    fn init_pool(&self) -> T;
}
