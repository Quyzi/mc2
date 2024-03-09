use self::{
    backend::{MemoryBackend, MemoryObjectID, MemoryObjectValue},
    error::MemoryError,
    transaction::MemoryTransaction,
};
use super::*;
use crate::{StorageShard, StorageTransaction};
use std::{
    cell::RefCell,
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Clone)]
pub struct MemoryShard {
    pub(crate) backend: MemoryBackend,
    pub(super) objects: RefCell<HashMap<MemoryObjectID, MemoryObjectValue>>,
}

impl<'b> StorageShard<'b, MemoryObjectValue, MemoryObjectID> for MemoryShard {
    type Error = Error;

    fn compare_swap(
        &self,
        key: MemoryObjectID,
        old: Option<MemoryObjectValue>,
        new: Option<MemoryObjectValue>,
    ) -> Result<Option<MemoryObjectValue>, Self::Error> {
        let mut objects = self.objects.try_borrow_mut()?;
        match (&old, &new) {
            // If old is None, create the value if it doesn't exist.
            (&None, &Some(_)) => {
                if objects.contains_key(&key) {
                    return Err(MemoryError::new(
                        "memoryshard/compare_swap",
                        &format!("failed to create new value with key {key}: value already exists"),
                    ));
                }
                let original = objects.insert(key, new.unwrap());
                Ok(original)
            }

            // If new is None, delete the value if old is correct.
            (&Some(_), &None) => {
                let original = objects.get(&key).cloned();
                if original == old {
                    let _ = objects.remove(&key);
                    Ok(original)
                } else {
                    Err(MemoryError::new(
                        "memoryshard/compare_swap",
                        &format!("failed to delete value with key {key}: old does not match new"),
                    ))
                }
            }

            // If both old and new are Some, modify the value if old is correct.
            (&Some(_), &Some(_)) => {
                let mut original = None;
                objects.entry(key).and_modify(|orig| {
                    if orig == &old.unwrap() {
                        original = Some(orig.clone());
                        *orig = new.unwrap();
                    }
                });
                Ok(original)
            }

            // If both old and new are None, delete the value.
            (&None, &None) => Ok(objects.remove(&key)),
        }
    }

    fn update_fetch<F>(
        &self,
        key: MemoryObjectID,
        mut f: F,
    ) -> Result<Option<MemoryObjectValue>, Self::Error>
    where
        F: FnMut(&mut MemoryObjectValue),
    {
        let mut objects = self.objects.try_borrow_mut()?;
        let mut original = None;
        objects.entry(key).and_modify(|v| {
            original = Some(v.clone());
            f(v);
        });
        Ok(original)
    }

    fn list(&self) -> Result<Box<dyn Iterator<Item = MemoryObjectID>>, Self::Error> {
        let objects = self.objects.try_borrow()?;
        Ok(Box::new(
            objects
                .keys()
                .copied()
                .collect::<Vec<MemoryObjectID>>()
                .into_iter(),
        ))
    }

    fn filter_list<F>(&self, f: F) -> Result<Box<dyn Iterator<Item = MemoryObjectID>>, Self::Error>
    where
        F: FnMut(&MemoryObjectID) -> Option<&MemoryObjectID>,
    {
        let objects = self.objects.try_borrow()?;
        Ok(Box::new(
            objects
                .keys()
                .filter_map(f)
                .copied()
                .collect::<Vec<MemoryObjectID>>()
                .into_iter(),
        ))
    }

    fn get(&self, key: MemoryObjectID) -> Result<Option<MemoryObjectValue>, Self::Error> {
        let objects = self.objects.try_borrow()?;
        match objects.get(&key) {
            Some(this) => Ok(Some(this.to_owned())),
            None => Ok(None),
        }
    }

    fn delete(&self, key: MemoryObjectID) -> Result<Option<MemoryObjectValue>, Self::Error> {
        let mut objects = self.objects.try_borrow_mut()?;
        Ok(objects.remove(&key))
    }

    fn compute_key(&self, value: &MemoryObjectValue) -> Result<MemoryObjectID, Self::Error> {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        Ok(hasher.finish())
    }

    fn prepare_value<T>(&self, value: &T) -> Result<MemoryObjectValue, Self::Error>
    where
        MemoryObjectValue: for<'a> TryFrom<&'a T, Error = MemoryError>,
    {
        MemoryObjectValue::try_from(value)
    }

    fn put(
        &self,
        key: MemoryObjectID,
        value: MemoryObjectValue,
    ) -> Result<
        Option<MemoryObjectValue>,
        <Self as StorageShard<'b, MemoryObjectValue, MemoryObjectID>>::Error,
    > {
        let mut objects = self.objects.try_borrow_mut()?;
        let mut old_value = None;

        objects
            .entry(key)
            .and_modify(|old| {
                old_value = Some(old.to_owned());
                *old = value.clone();
            })
            .or_insert(value);
        Ok(old_value)
    }

    fn new_transaction(
        &self,
    ) -> Result<
        impl StorageTransaction<'b, bytes::Bytes, u64>,
        <Self as StorageShard<'b, bytes::Bytes, u64>>::Error,
    > {
        Ok(MemoryTransaction {
            _backend: self.backend.clone(),
            shard: self.clone(),
            cache: RefCell::new(HashMap::new()),
        })
    }
}
