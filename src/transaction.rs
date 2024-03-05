use crate::prelude::*;
use std::future::Future;

pub trait Transaction<'t> {
    type Error;
    type ID: Identity;
    type Label: Label<'t>;

    fn get<O>(&mut self, id: Self::ID) -> Box<dyn Future<Output = Result<O, Self::Error>>>
        where O: StoreableItem;

    fn put<O>(&mut self, id: Self::ID, item: &O, meta: Vec<Self::Label>) -> Box<dyn Future<Output = Result<Self::ID, Self::Error>>>
        where O: StoreableItem;

    fn delete<O>(&mut self, id: Self::ID) -> Box<dyn Future<Output = Result<Option<O>, Self::Error>>>
        where O: StoreableItem;

    fn execute(self) -> Result<usize, Self::Error>;
}
