mod doc;
mod entity;
mod header;
mod syn;
mod xref;

use crate::error::Result;

/// Trait to convert an OBO graph element into an OBO syntax node.
#[cfg_attr(feature = "_doc", doc(cfg(feature = "obo")))]
pub trait FromGraph<T>: Sized {
    fn from_graph(source: T) -> Result<Self>;
}
