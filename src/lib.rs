#![cfg_attr(feature = "_doc", feature(doc_cfg, external_doc))]
#![cfg_attr(feature = "_doc", doc(include = "../README.md"))]
#![warn(clippy::all)]
#![recursion_limit="128"]
#![allow(unused_imports, dead_code)]

#[macro_use]
extern crate err_derive;
#[cfg(feature = "obo")]
#[macro_use]
extern crate mashup;
#[cfg(feature = "obo")]
extern crate fastobo;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

pub mod error;
pub mod model;
pub mod utils;
pub mod constants;

#[cfg(feature = "obo")]
mod from_graph;
#[cfg(feature = "obo")]
mod into_graph;

#[cfg(feature = "obo")]
pub use from_graph::FromGraph;
#[cfg(feature = "obo")]
pub use into_graph::IntoGraph;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use self::model::GraphDocument;
use self::error::Result;

#[inline]
pub fn from_str<S: AsRef<str>>(src: S) -> Result<GraphDocument> {
    serde_yaml::from_str::<model::GraphDocument>(src.as_ref()).map_err(From::from)
}

#[inline]
pub fn from_reader<R: Read>(r: R) -> Result<GraphDocument> {
    serde_yaml::from_reader::<R, model::GraphDocument>(r).map_err(From::from)
}

#[inline]
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<GraphDocument> {
    File::open(path)
        .map_err(From::from)
        .and_then(|r| serde_yaml::from_reader(r).map_err(From::from))
}


#[inline]
pub fn to_file<P: AsRef<Path>>(path: P, g: &GraphDocument) -> Result<()> {
    File::open(path)
        .map_err(From::from)
        .and_then(|r| serde_json::to_writer(r, g).map_err(From::from))
}
