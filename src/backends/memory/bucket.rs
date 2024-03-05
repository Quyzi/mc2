use std::cell::RefCell;

use super::{backend::MemoryBackend, ident::MemoryIdent, transaction::MemoryTransaction};
use crate::prelude::*;

#[derive(Clone)]
pub struct MemoryBucket {
    pub(crate) be: MemoryBackend,
}

impl Drop for MemoryBucket {
    fn drop(&mut self) {}
}

impl<'b> Bucket<'b> for MemoryBucket {
    type Error = anyhow::Error;
    type ID = MemoryIdent;
    type Backend = MemoryBackend;
    type Transaction = MemoryTransaction;

    fn close(self) -> Result<u32, Self::Error> {
        drop(self);
        Ok(0)
    }

    fn parent_backend(&self) -> Result<Self::Backend, Self::Error> {
        Ok(self.be.clone())
    }

    fn list(&self, _prefix: Option<Self::ID>) -> Result<Vec<Self::ID>, Self::Error> {
        todo!()
    }

    fn list_iter(
        &self,
        _prefix: Option<Self::ID>,
    ) -> Result<Box<dyn Iterator<Item = Self::ID>>, Self::Error> {
        todo!()
    }

    fn new_transaction(&self) -> Result<Self::Transaction, Self::Error> {
        let tx = MemoryTransaction {
            be: self.parent_backend()?,
            actions: RefCell::new(vec![]),
        };
        Ok(tx)
    }
}
