use serde::{Deserialize, Serialize};

pub trait Label<'l>: Clone + Serialize + Deserialize<'l> {
    type Error;
    type Output: Serialize + Deserialize<'l> + Clone;

    fn key_bytes(&self) -> Result<Self::Output, Self::Error>;
    fn key_bytes_inverse(&self) -> Result<Self::Output, Self::Error>;

    fn value_bytes(&self) -> Result<Self::Output, Self::Error>;
}
