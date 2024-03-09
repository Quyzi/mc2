pub mod backends;

#[cfg(test)]
pub mod tests;

pub trait StorageBackend<'b, B, K> {
    type Error;
    type Shard: StorageShard<'b, B, K>;

    fn open(path: &'b str) -> Result<Self, Self::Error>
    where
        Self: Sized + Clone;

    fn close(self) -> Result<usize, Self::Error>;

    fn sync(&self) -> Result<usize, Self::Error> {
        Ok(0)
    }

    fn open_shard(&self, name: &str) -> Result<Self::Shard, Self::Error>
    where
        Self::Shard: Sized + Clone;
}

pub trait StorageShard<'b, B, K> {
    type Error;

    fn compute_key(&self, value: &B) -> Result<K, Self::Error>;
    fn prepare_value<T>(&self, value: &T) -> Result<B, Self::Error>
    where
        B: for<'a> TryFrom<&'a T, Error = Self::Error>;

    fn list(&self) -> Result<Box<dyn Iterator<Item = K>>, Self::Error>;
    fn filter_list<F>(&self, f: F) -> Result<Box<dyn Iterator<Item = K>>, Self::Error>
    where
        F: FnMut(&K) -> Option<&K>;

    fn get(&self, key: K) -> Result<Option<B>, Self::Error>;
    fn delete(&self, key: K) -> Result<Option<B>, Self::Error>;
    fn put(&self, key: K, value: B) -> Result<Option<B>, Self::Error>;

    fn compare_swap(
        &self,
        key: K,
        old: Option<B>,
        new: Option<B>,
    ) -> Result<Option<B>, Self::Error>;

    fn update_fetch<F>(&self, key: K, f: F) -> Result<Option<B>, Self::Error>
    where
        F: FnMut(&mut B);

    fn new_transaction(&self) -> Result<impl StorageTransaction<'b, B, K>, Self::Error>;
}

pub trait StorageTransaction<'b, B, K> {
    type Error;

    fn get(&self, key: K) -> Result<Option<B>, Self::Error>;
    fn delete(&self, key: K) -> Result<Option<B>, Self::Error>;
    fn put(&self, key: K, value: B) -> Result<Option<B>, Self::Error>;

    fn compare_swap(
        &self,
        key: K,
        old: Option<B>,
        new: Option<B>,
    ) -> Result<Option<B>, Self::Error>;

    fn update_fetch<F>(&self, key: K, f: F) -> Result<Option<B>, Self::Error>
    where
        F: FnMut(&mut B);

    fn commit(&self) -> Result<usize, Self::Error>;
    fn reset(&self) -> Result<usize, Self::Error>;
}
