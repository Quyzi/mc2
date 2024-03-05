use crate::prelude::*;
use bytes::Bytes;
use std::{cell::RefCell, collections::HashMap, path::PathBuf};

use super::{bucket::MemoryBucket, ident::MemoryIdent};

#[derive(Clone)]
pub struct MemoryBackend {
    pub(crate) given_path: PathBuf,
    pub(crate) objects: RefCell<HashMap<MemoryIdent, Bytes>>,
    pub(crate) labels: RefCell<HashMap<String, Vec<MemoryIdent>>>,
    pub(crate) object_labels: RefCell<HashMap<MemoryIdent, Vec<String>>>,
}

impl<'b> Backend<'b> for MemoryBackend {
    type Error = anyhow::Error;
    type Bucket = MemoryBucket;

    fn open(p: PathBuf) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            given_path: p,
            objects: RefCell::new(HashMap::new()),
            labels: RefCell::new(HashMap::new()),
            object_labels: RefCell::new(HashMap::new()),
        })
    }

    fn close(self) -> Result<u32, Self::Error> {
        Ok(0)
    }

    fn sync(&self) -> Result<u32, Self::Error> {
        Ok(0)
    }

    fn open_bucket(&self, _name: &str) -> Result<Self::Bucket, Self::Error> {
        let b = MemoryBucket { be: self.clone() };
        Ok(b)
    }
}
