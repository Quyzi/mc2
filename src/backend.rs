use std::path::PathBuf;

use crate::prelude::Bucket;

pub trait Backend<'b>: Clone {
    type Error;
    type Bucket: Bucket<'b>;

    fn open(p: PathBuf) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn close(self) -> Result<u32, Self::Error>;
    fn sync(&self) -> Result<u32, Self::Error>;

    fn open_bucket(&self, name: &str) -> Result<Self::Bucket, Self::Error>;
}
