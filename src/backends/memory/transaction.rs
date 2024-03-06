use std::cell::RefCell;

use self::{
    backend::{MemoryObjectID, MemoryObjectValue}, error::MemoryError, shard::MemoryShard
};
use super::*;
use crate::{StorageShard, StorageTransaction};
use futures::{future::ready, prelude::future::BoxFuture};

pub enum TransactionAction {
    Get(Box<dyn Fn(MemoryObjectID, MemoryObjectValue) -> Result<MemoryObjectValue, MemoryError>>),
}


pub struct MemoryTransaction {
    pub(super) shard: MemoryShard,
    #[allow(dead_code)]
    pub(super) id: String,
    pub(super) actions: RefCell<Vec<TransactionAction>>,
}

impl<'b> StorageTransaction<'b, MemoryObjectValue, MemoryObjectID> for MemoryTransaction {
    type Error = Error;

    fn get(&self, key: MemoryObjectID) -> BoxFuture<Result<MemoryObjectValue, Self::Error>> {
        Box::pin(ready(self.shard.get(key)))
    }

    fn put(
        &self,
        key: MemoryObjectID,
        value: MemoryObjectValue,
    ) -> BoxFuture<Result<Option<MemoryObjectValue>, Self::Error>> {
        Box::pin(ready(self.shard.put(key, value)))
    }

    fn delete(
        &self,
        key: MemoryObjectID,
    ) -> BoxFuture<Result<Option<MemoryObjectValue>, Self::Error>> {
        Box::pin(ready(self.shard.delete(key)))
    }

    fn compare_swap(
        &self,
        key: MemoryObjectID,
        old: Option<MemoryObjectValue>,
        new: Option<MemoryObjectValue>,
    ) -> BoxFuture<Result<Option<MemoryObjectValue>, Self::Error>> {
        Box::pin(ready(self.shard.compare_swap(key, old, new)))
    }

    fn update_fetch<F>(
        &self,
        key: MemoryObjectID,
        f: F,
    ) -> BoxFuture<Result<Option<MemoryObjectValue>, Self::Error>>
    where
        F: FnMut(&mut MemoryObjectValue),
    {
        Box::pin(ready(self.shard.update_fetch(key, f)))
    }
}
