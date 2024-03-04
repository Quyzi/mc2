use crate::{backend::Backend, ident::Ident, storeable::StoreableItem};

pub trait Bucket<'b> {
    type Error: std::error::Error;
    type ID: Ident<'b>;

    fn open(be: &impl Backend, name: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn close(self) -> Result<u32, Self::Error>;

    fn list(&self, prefix: Option<Self::ID>) -> Result<Vec<Self::ID>, Self::Error>;
    fn get(&self, id: Self::ID) -> Result<Option<impl StoreableItem>, Self::Error>;
    fn put(&self, id: Self::ID, item: &impl StoreableItem<'b>) -> Result<Option<u32>, Self::Error>;
    fn delete(&self, id: Self::ID) -> Result<Option<impl StoreableItem<'b>>, Self::Error>;
}
