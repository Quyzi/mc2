use bytes::Bytes;
use crate::prelude::*;

/// A StoreableItem is anything that can be made into or made from a Bytes
pub trait StoreableItem: TryFrom<Bytes> + TryInto<Bytes> {
    type Error;

    fn to_bytes(&self) -> Result<Bytes, <Self as StoreableItem>::Error>;

    fn from_bytes(bs: Bytes) -> Result<Self, <Self as StoreableItem>::Error>
    where
        Self: Sized;
}

impl<T> StoreableItem for T
where
    T: TryFrom<Bytes> + TryInto<Bytes>,
    for<'a> Bytes: TryFrom<&'a T, Error = StorageError>,
    for<'a> Bytes: TryInto<T, Error = StorageError>,
{
    type Error = StorageError;

    fn to_bytes(&self) -> Result<Bytes, <Self as StoreableItem>::Error> {
        match self.try_into() {
            Ok(this) => Ok(this),
            Err(e) => Err(StorageError::new("StoreableItem/to_bytes", &e.to_string())),
        }
    }

    fn from_bytes(bs: Bytes) -> Result<Self, <Self as StoreableItem>::Error>
    where
        Self: Sized,
    {
        match bs.try_into() {
            Ok(this) => Ok(this),
            Err(e) => Err(StorageError::new("StoreableItem/from_bytes", &e.to_string())),
        }
    }
}
