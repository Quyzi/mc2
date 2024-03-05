use std::{cell::RefCell, collections::HashMap};
use crate::prelude::*;
use super::prelude::*;

#[derive(Clone)]
pub struct MemoryBackend {
    pub(super) storage: RefCell<HashMap<MemoryIdentity, ()>>,
}