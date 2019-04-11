use try_from::TryFrom;

use crate::{Error, ErrorKind, Result};

pub trait ParseIterator: Iterator {
    fn try_from_next<R, E>(&mut self) -> Result<R>
    where
        R: TryFrom<<Self as Iterator>::Item, Err = E>,
        Error: From<E>;
}

impl<T> ParseIterator for T
where
    T: Iterator,
{
    fn try_from_next<R, E>(&mut self) -> Result<R>
    where
        R: TryFrom<<Self as Iterator>::Item, Err = E>,
        Error: From<E>,
    {
        let value = self.next().ok_or_else(|| Error::new(ErrorKind::Parse))?;

        try_from::TryFrom::try_from(value).map_err(Into::into)
    }
}