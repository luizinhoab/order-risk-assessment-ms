pub trait Repository<T, K> {
    fn save(&self, object: T) -> Result<T, K>;
}
