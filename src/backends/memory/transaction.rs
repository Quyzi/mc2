use std::cell::RefCell;

use crate::prelude::*;

use super::{backend::MemoryBackend, ident::MemoryIdent, label::MemoryLabel};

#[derive(Clone)]
pub(crate) enum Action {
    Get(MemoryIdent),
    Put(MemoryIdent),
    Delete(MemoryIdent),
}

pub struct MemoryTransaction {
    pub(crate) be: MemoryBackend,
    pub(crate) actions: RefCell<Vec<Action>>,
}

impl<'t> Transaction<'t> for MemoryTransaction {
    type Error = anyhow::Error;
    type ID = MemoryIdent;
    type Label = MemoryLabel;

    fn get<O>(
        &mut self,
        _id: Self::ID,
    ) -> Box<dyn std::future::Future<Output = Result<O, Self::Error>>> {
        todo!()
    }

    fn put<O>(
        &mut self,
        _id: Self::ID,
        _item: &O,
        _meta: Vec<Self::Label>,
    ) -> Box<dyn std::future::Future<Output = Result<Self::ID, Self::Error>>> {
        todo!()
    }

    fn delete<O>(
        &mut self,
        _id: Self::ID,
    ) -> Box<dyn std::future::Future<Output = Result<Option<O>, Self::Error>>> {
        todo!()
    }

    fn execute(self) -> Result<usize, Self::Error> {
        todo!()
    }
}
