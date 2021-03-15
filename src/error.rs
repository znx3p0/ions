use serde::ser::{Error, StdError};

#[derive(Debug)]
pub struct Err;

impl std::fmt::Display for Err {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Error for Err {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        unimplemented!()
    }
}

impl serde::de::Error for Err {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        unimplemented!()
    }
}

impl StdError for Err {}
