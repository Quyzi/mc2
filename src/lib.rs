use std::future::Future;

pub trait StorageBackend<'b, B, K> {
    type Error;
    type Transaction: StorageTransaction<'b, B, K>;
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

    fn start_transaction(&self, id: &str, shard: &str) -> Result<Self::Transaction, Self::Error>
    where
        Self::Transaction: Sized + Clone;
}

pub trait StorageShard<'b, B, K> {
    type Error;

    fn list(&self) -> Result<impl Iterator<Item = K>, Self::Error>;
    fn filter_list<F>(&self, f: F) -> Result<impl Iterator<Item = K>, Self::Error>
    where
        F: FnMut(K) -> Result<K, Self::Error>;

    fn get_one(&self, key: K) -> Result<B, Self::Error>;
    fn get_many(&self, keys: Vec<K>) -> Vec<Result<B, Self::Error>>;

    fn put(&self, key: K, value: B) -> Result<Option<B>, Self::Error>;

    fn delete_one(&self, key: K) -> Result<Option<B>, Self::Error>;
    fn delete_many(&self, keys: Vec<K>) -> Vec<(K, Result<Option<B>, Self::Error>)>;

    fn compare_swap(
        &self,
        key: K,
        old: Option<B>,
        new: Option<B>,
    ) -> Result<Option<B>, Self::Error>;
    
    fn update_fetch<F>(&self, key: K, f: F) -> Result<Option<B>, Self::Error>
    where
        F: FnMut(Option<B>) -> Option<B>;
}

pub trait StorageTransaction<'b, B, K> {
    type Error;

    fn get_one(&self, key: K) -> Box<dyn Future<Output = Result<B, Self::Error>>>;
    fn get_many(
        &self,
        keys: Vec<K>,
    ) -> Box<dyn Future<Output = Vec<(u64, Result<B, Self::Error>)>>>;

    fn put(&self, key: K, value: B) -> Box<dyn Future<Output = Result<Option<B>, Self::Error>>>;

    fn delete_one(&self, key: K) -> Box<dyn Future<Output = Result<Option<B>, Self::Error>>>;
    fn delete_many(
        &self,
        keys: Vec<K>,
    ) -> Box<dyn Future<Output = Vec<Result<(K, Option<B>), Self::Error>>>>;

    fn compare_swap(
        &self,
        key: K,
        old: Option<B>,
        new: Option<B>,
    ) -> Box<dyn Future<Output = Result<Option<B>, Self::Error>>>;

    fn update_fetch<F>(
        &self,
        key: K,
        f: F,
    ) -> Box<dyn Future<Output = Result<Option<B>, Self::Error>>>
    where
        F: FnMut(Option<B>) -> Option<B>;
}
