use std::error::Error;

mod generator;
mod memorydb;
mod rocksdb;
pub use generator::*;

pub trait DB: Send + Sync {
    type Error: Error;

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Self::Error>;

    fn contains(&self, key: &[u8]) -> Result<bool, Self::Error>;

    /// Insert data into the cache.
    fn insert(&self, key: Vec<u8>, value: Vec<u8>) -> Result<(), Self::Error>;

    /// Insert data into the cache.
    fn remove(&self, key: &[u8]) -> Result<(), Self::Error>;

    /// Insert a batch of data into the cache.
    fn insert_batch(&self, keys: Vec<Vec<u8>>, values: Vec<Vec<u8>>) -> Result<(), Self::Error> {
        for i in 0..keys.len() {
            let key = keys[i].clone();
            let value = values[i].clone();
            self.insert(key, value)?;
        }
        Ok(())
    }

    /// Remove a batch of data into the cache.
    fn remove_batch(&self, keys: &[Vec<u8>]) -> Result<(), Self::Error> {
        for key in keys {
            self.remove(key)?;
        }
        Ok(())
    }

    fn remove_prefix(&self, prefix: &[u8]) -> Result<(), Self::Error>;

    /// Flush data to the DB from the cache.
    fn flush(&self) -> Result<(), Self::Error>;

    #[cfg(test)]
    fn len(&self) -> Result<usize, Self::Error>;
    #[cfg(test)]
    fn is_empty(&self) -> Result<bool, Self::Error>;
}
