use std::{cell::RefCell, collections::HashMap};

use crate::{StorageShard, StorageTransaction};

use super::{
    backend::{MemoryBackend, MemoryObjectID, MemoryObjectValue},
    error::MemoryError,
    shard::MemoryShard,
};

#[derive(Clone)]
pub struct MemoryTransaction {
    pub(crate) _backend: MemoryBackend,
    pub(crate) shard: MemoryShard,
    pub(crate) cache: RefCell<HashMap<MemoryObjectID, Option<MemoryObjectValue>>>,
}

impl<'b> StorageTransaction<'b, MemoryObjectValue, MemoryObjectID> for MemoryTransaction {
    type Error = MemoryError;

    fn get(&self, key: MemoryObjectID) -> Result<Option<MemoryObjectValue>, Self::Error> {
        let cache = self.cache.try_borrow()?;
        match cache.get(&key) {
            Some(Some(cached)) => Ok(Some(cached.clone())),
            Some(None) => Ok(None),
            None => Ok(self.shard.get(key)?),
        }
    }

    fn put(
        &self,
        key: MemoryObjectID,
        value: MemoryObjectValue,
    ) -> Result<Option<MemoryObjectValue>, Self::Error> {
        let mut cache = self.cache.try_borrow_mut()?;
        match cache.insert(key, Some(value)) {
            Some(cached) => Ok(cached),
            None => Ok(self.shard.get(key)?),
        }
    }

    fn delete(&self, key: MemoryObjectID) -> Result<Option<MemoryObjectValue>, Self::Error> {
        let mut cache = self.cache.try_borrow_mut()?;
        match cache.remove(&key) {
            Some(cached) => Ok(cached),
            None => Ok(self.shard.delete(key)?),
        }
    }

    fn compare_swap(
        &self,
        key: MemoryObjectID,
        old: Option<MemoryObjectValue>,
        new: Option<MemoryObjectValue>,
    ) -> Result<Option<MemoryObjectValue>, Self::Error> {
        let mut cache = self.cache.try_borrow_mut()?;
        let (original, matches_old) = match cache.get(&key) {
            Some(cached) => (cached.clone(), (cached == &old)),
            None => (self.shard.get(key)?, old.is_none()),
        };

        match (&old, &new) {
            // If old is None and new is Some, create a value if it doesn't exist.
            // Return an error if value already exists.
            (None, Some(_)) => {
                if original.is_some() {
                    Err(MemoryError::new(
                        "memorytransaction/compare_swap",
                        &format!("failed to create new value with key {key}: value already exists"),
                    ))
                } else {
                    let _ = cache.insert(key, new);
                    Ok(original)
                }
            }

            // If old is Some and new is None, delete the value if old matches stored.
            // Return an error if values don't match.
            (Some(_), None) => {
                if matches_old {
                    let _ = cache.insert(key, None);
                    Ok(original)
                } else {
                    Err(MemoryError::new(
                        "memorytransaction/compare_swap",
                        &format!(
                            "failed to delete value for {key}: given value does not match stored"
                        ),
                    ))
                }
            }

            // If both old and new are Some, modify the value if old matches stored
            (Some(_), Some(_)) => {
                if matches_old {
                    let _ = cache.insert(key, new);
                    Ok(original)
                } else {
                    Err(MemoryError::new(
                        "memorytransaction/compare_swap",
                        &format!(
                            "failed to update value for {key}: given value does not match stored"
                        ),
                    ))
                }
            }

            // If both old and new are None, delete the value.
            (None, None) => {
                let _ = cache.insert(key, new);
                Ok(original)
            }
        }
    }

    /// Fetch the value, apply a function to it and return the result.
    fn update_fetch<F>(
        &self,
        key: MemoryObjectID,
        mut f: F,
    ) -> Result<Option<MemoryObjectValue>, Self::Error>
    where
        F: FnMut(&mut MemoryObjectValue),
    {
        let mut cache = self.cache.try_borrow_mut()?;

        let mut value = match cache.get(&key) {
            Some(Some(cached)) => cached.clone(),
            Some(None) => {
                return Err(MemoryError::new(
                    "memorytransaction/update_fetch",
                    &format!("failed to update_fetch key {key}: object was deleted in transaction"),
                ))
            }
            None => match self.shard.get(key)? {
                Some(stored) => stored,
                None => {
                    return Err(MemoryError::new(
                        "memorytransaction/update_fetch",
                        &format!("failed to update_fetch key {key}: key not found"),
                    ))
                }
            },
        };

        f(&mut value);
        let _ = cache.insert(key, Some(value.clone()));

        Ok(Some(value))
    }

    fn commit(&self) -> Result<usize, Self::Error> {
        let cache = self.cache.try_borrow()?;

        for key in cache.keys() {
            let value = match cache.get(key) {
                Some(tx_stored) => tx_stored.clone(),
                None => {
                    return Err(MemoryError::new(
                        "memorytransaction/commit",
                        &format!("failed to apply transaction: key {key} not found"),
                    ))
                }
            };
            if value.is_none() {
                let _ = self.shard.delete(*key)?;
            } else {
                let _ = self.shard.put(*key, value.unwrap())?;
            }
        }
        Ok(cache.len())
    }

    fn reset(&self) -> Result<usize, Self::Error> {
        let mut cache = self.cache.try_borrow_mut()?;

        let deleted = cache.len();
        *cache = HashMap::new();
        Ok(deleted)
    }
}
