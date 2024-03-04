use std::path::PathBuf;

pub trait Backend: Clone {
    type Error: std::error::Error;

    fn open(p: PathBuf) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn close(self) -> Result<u32, Self::Error>;

    fn sync(&self) -> Result<u32, Self::Error>;
}
