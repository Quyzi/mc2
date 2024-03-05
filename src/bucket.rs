use crate::prelude::*;

pub trait Bucket<'b> {
    type Error;
    type ID: Identity;
    type Backend: Backend<'b>;
    type Transaction: Transaction<'b>;

    /// Close this Bucket.  This should not delete data.
    fn close(self) -> Result<usize, Self::Error>;

    /// Get the Backend behind this Bucket. 
    fn parent_backend(&self) -> Result<Self::Backend, Self::Error>;

    /// List objects stored in this bucket by id with an optional prefix
    fn list(&self, prefix: Option<Self::ID>) -> Result<Vec<Self::ID>, Self::Error>;

    /// Get an iterator over all object ids stored in this bucket with an optional prefix
    fn list_iter(&self, prefix: Option<Self::ID>) -> Result<Box<dyn Iterator<Item = Self::ID>>, Self::Error>;

    /// Start a new transaction
    fn new_transaction(&self) -> Result<Self::Transaction, Self::Error>;
}
