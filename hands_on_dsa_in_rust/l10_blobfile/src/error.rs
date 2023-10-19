use failure_derive::*;

#[derive(Debug, Fail)]
pub enum BlobError {
    #[fail(display = "No Room")]
    NoRoom,
    #[fail(display = "Too Big {}", 0)]
    TooBig(u64),
    #[fail(display = "Not Found")]
    NotFound,
    #[fail(display = "Bincode {}", 0)]
    Bincode(bincode::Error),
    #[fail(display = "IO {}", 0)]
    IO(std::io::Error)
}

impl From<bincode::Error> for BlobError {
    fn from(e: bincode::Error) -> Self {
        BlobError::Bincode(e)
    }
}

impl From<std::io::Error> for BlobError {
    fn from(e: std::io::Error) -> Self {
        BlobError::IO(e)
    }
}