use std::path::PathBuf;

use crate::prelude::*;

pub trait Backend<'b>: Clone {
    type Error;
    type Bucket: Bucket<'b>;

    /// Open the backend
    fn open(p: PathBuf) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Close the backend
    fn close(self) -> Result<usize, Self::Error>;

    /// (optional) Sync the backend to permanent storage
    fn sync(&self) -> Result<usize, Self::Error> {
        Ok(0)
    }

    /// Get a storage bucket by name
    fn open_bucket(&self, name: &str) -> Result<Self::Bucket, Self::Error>;

    /// Get an iterator of all storage bucket names
    fn list_buckets(&self) -> Result<impl Iterator<Item = String>, Self::Error>;

    /// Drop a bucket by name
    fn drop_bucket(&self, name: &str) -> Result<usize, Self::Error>;
}
