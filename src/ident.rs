use serde::{Deserialize, Serialize};

use crate::storeable::StoreableItem;

pub trait Ident<'i>: Clone + Serialize + Deserialize<'i> {
    type Error;

    fn from_storeable(item: &impl StoreableItem<'i>) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized;
}
