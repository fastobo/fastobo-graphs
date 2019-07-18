mod doc;
mod entity;
mod header;
mod syn;
mod xref;

use crate::error::Result;

pub trait FromGraph<T>: Sized {
    fn from_graph(source: T) -> Result<Self>;
}
