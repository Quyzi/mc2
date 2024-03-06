use crate::StorageBackend;
use bytes::Bytes;
use std::{cell::RefCell, collections::HashMap};

use self::{error::MemoryError, shard::MemoryShard, transaction::MemoryTransaction};
use super::*;

pub(super) type MemoryObjectID = u64;
pub(super) type MemoryObjectValue = Bytes;

#[derive(Clone)]
pub struct MemoryBackend {
    shards: RefCell<HashMap<String, RefCell<HashMap<MemoryObjectID, MemoryObjectValue>>>>,
}

impl Drop for MemoryBackend {
    fn drop(&mut self) {
        
    }
}

impl<'b> StorageBackend<'b, MemoryObjectValue, MemoryObjectID> for MemoryBackend {
    type Error = MemoryError;
    // type Transaction = MemoryTransaction;
    type Shard = MemoryShard;

    fn open(_path: &'b str) -> Result<Self, Self::Error>
    where
        Self: Sized + Clone,
    {
        Ok(Self {
            shards: RefCell::new(HashMap::new()),
        })
    }

    fn close(self) -> Result<usize, Self::Error> {
        drop(self);
        Ok(0)
    }

    fn open_shard(&self, name: &str) -> Result<Self::Shard, Self::Error>
    where
        Self::Shard: Sized + Clone,
    {
        let mut shards = self.shards.try_borrow_mut()?;
        let shard = match shards.get(name) {
            Some(s) => s.clone(),
            None => {
                let this = RefCell::new(HashMap::new());
                shards.insert(name.to_string(), this.clone());
                this
            }
        };
        Ok(MemoryShard { objects: shard })
    }

    // fn start_transaction(&self, id: &str, _shard: &str) -> Result<Self::Transaction, Self::Error>
    // where
    //     Self::Transaction: Sized,
    // {
    //     Ok(MemoryTransaction {
    //         id: id.to_string(),
    //         shard: self.open_shard(id)?,
    //         actions: RefCell::new(vec![]),
    //     })
    // }
}
