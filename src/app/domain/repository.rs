#[cfg(test)]
use mockall::*;

#[cfg_attr(test, automock)]
pub trait Repository<T: 'static, K: 'static> {
    fn save(&self, object: T) -> Result<T, K>;
}
