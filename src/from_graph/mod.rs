mod doc;
mod entity;
mod header;
mod syn;
mod xref;

pub trait FromGraph<T> {
    fn from_graph(source: T) -> Self;
}
