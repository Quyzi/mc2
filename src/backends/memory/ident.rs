use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct MemoryIdent(u64);

impl<'i> Ident<'i> for MemoryIdent {
    type Error = anyhow::Error;

    fn from_storeable(_item: &impl StoreableItem<'i>) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized,
    {
        todo!()
    }
}
