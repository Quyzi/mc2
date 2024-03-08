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

    fn open_shard(
        &self,
        name: &str,
    ) -> Result<Self::Shard, Self::Error>
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

    fn get(&self, key: K) -> Result<B, Self::Error>;
    fn delete(&self, key: K) -> Result<Option<B>, Self::Error>;
    fn put<T>(&self, key: K, value: T) -> Result<Option<B>, Self::Error>
    where
        T: Storeable<'b, B>,
        B: for<'a> TryFrom<&'a T>;

    fn compare_swap(
        &self,
        key: K,
        old: Option<B>,
        new: Option<B>,
    ) -> Result<Option<B>, Self::Error>;

    fn update_fetch<F>(&self, key: K, f: F) -> Result<Option<B>, Self::Error>
    where
        F: FnMut(&mut B);
}

pub trait Storeable<'b, B>: Sized
where
    B: for<'a> TryFrom<&'a Self>,
    Self: for<'a> TryFrom<&'a B>,
{
    type Error;

    fn to_storeable(&self) -> Result<B, <Self as Storeable<'b, B>>::Error>;

    fn from_storeable(st: B) -> Result<Self, <Self as Storeable<'b, B>>::Error>
    where
        Self: Sized;
}
