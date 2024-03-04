use crate::storeable::StoreableItem;

pub trait Ident<'i>: Clone {
    type Error: std::error::Error;
    type Serializer: serde::Serializer;

    fn from_storeable(item: &impl StoreableItem<'i>) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized;
}
