#![recursion_limit="128"]
#![allow(unused_imports, dead_code)]

#[cfg(feature = "obo")]
#[macro_use]
extern crate mashup;
extern crate serde;
#[cfg(feature = "obo")]
extern crate fastobo;

mod model;
mod utils;
mod constants;

#[cfg(feature = "obo")]
mod from_graph;
#[cfg(feature = "obo")]
mod into_graph;

pub use model::*;

#[cfg(feature = "obo")]
pub use from_graph::FromGraph;
#[cfg(feature = "obo")]
pub use into_graph::IntoGraphCtx;
