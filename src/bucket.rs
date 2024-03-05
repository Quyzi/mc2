use crate::prelude::*;

pub trait Bucket<'b> {
    type Error;
    type ID: Ident<'b>;
    type Backend: Backend<'b>;
    type Transaction: Transaction<'b>;

    fn close(self) -> Result<u32, Self::Error>;
    fn parent_backend(&self) -> Result<Self::Backend, Self::Error>;

    fn list(&self, prefix: Option<Self::ID>) -> Result<Vec<Self::ID>, Self::Error>;
    fn list_iter(
        &self,
        prefix: Option<Self::ID>,
    ) -> Result<Box<dyn Iterator<Item = Self::ID>>, Self::Error>;

    fn new_transaction(&self) -> Result<Self::Transaction, Self::Error>;
}
