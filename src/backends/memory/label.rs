use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct MemoryLabel(String, String);

impl<'l> Label<'l> for MemoryLabel {
    type Error = anyhow::Error;
    type Output = String;

    fn key_bytes(&self) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn key_bytes_inverse(&self) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn value_bytes(&self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
