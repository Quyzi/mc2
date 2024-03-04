use serde::{Deserialize, Serialize};

pub trait StoreableItem<'i>: Serialize + Deserialize<'i> + Clone {
    type Output: Serialize + Deserialize<'i> + Clone;
    type Error: std::error::Error;
    type Serializer: serde::Serializer;

    fn to_storeable(&self) -> Result<Self::Output, Self::Error>;
    fn from_storeable(item: Self::Output) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
