use std::hash::{DefaultHasher, Hash, Hasher};

use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::prelude::*;
use super::prelude::*;

/// Wrapper for u64 to use hash of object as the ID
#[derive(Clone, Copy, Debug, Hash)]
pub struct MemoryIdentity(u64);

impl TryFrom<Bytes> for MemoryIdentity {
    type Error = StorageError;

    fn try_from(mut value: Bytes) -> Result<Self, Self::Error> {
        let this = &value.get_u64();
        Ok(Self(*this))
    }
}

impl TryInto<Bytes> for MemoryIdentity {
    type Error = StorageError;

    fn try_into(self) -> Result<Bytes, Self::Error> {
        let mut this = BytesMut::with_capacity(64);
        this.put_u64(self.0);
        Ok(this.freeze())
    }
}

impl TryFrom<&MemoryIdentity> for Bytes {
    type Error = StorageError;

    fn try_from(value: &MemoryIdentity) -> Result<Self, Self::Error> {
        let mut this = BytesMut::with_capacity(64);
        this.put_u64(value.0);
        Ok(this.freeze())
    }
}

impl Identity for MemoryIdentity
{
    type Error = StorageError;

    fn from_item(i: impl StoreableItem) -> Result<Self, <Self as StoreableItem>::Error>
        where Self: Sized {
        let mut hasher = DefaultHasher::new();
        let bs = i.to_bytes().map_err(|_e| StorageError::new("MemoryIdentity/from_item", ""))?;
        bs.hash(&mut hasher);
        Ok(Self(hasher.finish()))
    }
}