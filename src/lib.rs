#![cfg_attr(feature = "_doc", feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![warn(clippy::all)]
#![recursion_limit = "256"]
#![allow(unused_variables, dead_code)]

#[macro_use]
extern crate thiserror;
#[cfg(feature = "obo")]
#[macro_use]
extern crate mashup;
#[cfg(feature = "obo")]
extern crate fastobo;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

pub mod constants;
pub mod error;
#[cfg(feature = "obo")]
mod from_graph;
#[cfg(feature = "obo")]
mod into_graph;
pub mod model;
mod utils;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use self::error::Result;
#[cfg(feature = "obo")]
pub use self::from_graph::FromGraph;
#[cfg(feature = "obo")]
pub use self::into_graph::IntoGraph;
use self::model::GraphDocument;

// ---------------------------------------------------------------------------

/// Read an OBO graph from a string containing a JSON or YAML serialization.
#[inline]
pub fn from_str<S: AsRef<str>>(src: S) -> Result<GraphDocument> {
    serde_yaml::from_str::<model::GraphDocument>(src.as_ref()).map_err(From::from)
}

/// Read an OBO graph from a `Read` implementor.
#[inline]
pub fn from_reader<R: Read>(r: R) -> Result<GraphDocument> {
    serde_yaml::from_reader::<R, model::GraphDocument>(r).map_err(From::from)
}

/// Read an OBO graph from a file on the local filesystem.
#[inline]
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<GraphDocument> {
    File::open(path)
        .map_err(From::from)
        .and_then(|r| serde_yaml::from_reader(r).map_err(From::from))
}

// ---------------------------------------------------------------------------

/// Write an OBO graph to a string.
#[inline]
pub fn to_string(g: &GraphDocument) -> Result<String> {
    serde_json::to_string(g).map_err(From::from)
}

/// Write an OBO graph to a `Write` implementor.
#[inline]
pub fn to_writer<W: Write>(w: W, g: &GraphDocument) -> Result<()> {
    serde_json::to_writer(w, g).map_err(From::from)
}

/// Write an OBO graph to a file on the local filesystem.
#[inline]
pub fn to_file<P: AsRef<Path>>(path: P, g: &GraphDocument) -> Result<()> {
    File::create(path)
        .map_err(From::from)
        .and_then(|w| to_writer(w, g))
}
