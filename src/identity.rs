use crate::prelude::*;

pub trait Identity: StoreableItem + Sized {
    type Error;

    fn from_item(i: impl StoreableItem) -> Result<Self, <Self as StoreableItem>::Error>
        where Self: Sized;

}
