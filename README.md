# `fastobo-graphs` [![Star me](https://img.shields.io/github/stars/fastobo/fastobo-graphs.svg?style=social&label=Star&maxAge=3600)](https://github.com/fastobo/fastobo-graphs/stargazers)

*[OBO Graphs](https://github.com/geneontology/obographs/) decoder and encoder in Rust.*

[![Actions](https://img.shields.io/github/workflow/status/fastobo/fastobo-graphs/Test?style=flat-square&maxAge=600)](https://github.com/fastobo/fastobo-graphs/actions)
[![Codecov](https://img.shields.io/codecov/c/gh/fastobo/fastobo-graphs/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/fastobo/fastobo-graphs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/fastobo/fastobo-graphs)
[![Crate](https://img.shields.io/crates/v/fastobo-graphs.svg?maxAge=600&style=flat-square)](https://crates.io/crates/fastobo-graphs)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/fastobo-graphs)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/fastobo/fastobo-graphs/blob/master/CHANGELOG.md)
[![GitHub issues](https://img.shields.io/github/issues/fastobo/fastobo-graphs.svg?style=flat-square)](https://github.com/fastobo/fastobo-graphs/issues)
[![DOI](https://img.shields.io/badge/doi-10.7490%2Ff1000research.1117405.1-brightgreen?style=flat-square&maxAge=31536000)](https://f1000research.com/posters/8-1500)


## Overview

This library provides an implementation of the
[OBO Graphs schema](https://github.com/geneontology/obographs/) specified by
the Gene Ontology to provide developers with a data format easier to use than
plain ontology files in [OBO](http://owlcollab.github.io/oboformat/doc/obo-syntax.html)
or [OWL](https://www.w3.org/TR/owl2-syntax/) format.

* **Data structures** - the complete OBO Graphs schema is reproduced into Rust
  data structures with public fields for direct access to the graphs nodes. See
  [`fastobo_graphs::model`](https://docs.rs/fastobo-graphs/latest/fastobo_graphs/model/index.html)
  to see the comprehensive list of data structures.
* **I/O** - structures use [`serde`](https://docs.rs/serde) to implement
  serialization and deserialization from both YAML and JSON.
* **Errors** - fallible operations can return an
  [`Error`](https://docs.rs/fastobo-graphs/latest/fastobo_graphs/error/enum.Error.html)
  with informative messages as well as an
  [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
  implementation.
* **Conversion traits** - OBO Graphs can be (partially) converted to and from
  plain OBO documents, using the
  [`FromGraph`](https://docs.rs/fastobo-graphs/latest/fastobo_graphs/trait.FromGraph.html) and
  [`IntoGraph`](https://docs.rs/fastobo-graphs/latest/fastobo_graphs/trait.IntoGraph.html) traits.

## Usage

Add `fastobo_graphs` to the `[dependencies]` sections of your `Cargo.toml`
manifest:
```toml
[dependencies]
fastobo-graphs = "0.4"
```

Then use one of the top-level functions in `fastobo_graphs` to load a JSON or
YAML serialized OBO Graph:
```rust
extern crate fastobo_graphs;
extern crate ureq;

fn main() {
    let response = ureq::get("http://purl.obolibrary.org/obo/zeco.json").call();

    match fastobo_graphs::from_reader(response.unwrap().into_reader()) {
        Ok(doc) => println!("Number of ZECO nodes: {}", doc.graphs[0].nodes.len()),
        Err(e) => panic!("Could not parse ZECO OBO Graph: {}", e),
    }
}
```

## Features

The following feature is enabled by default, but can be disabled if needed:

* ***obo*** - compile
  [`FromGraph`](https://docs.rs/fastobo-graphs/latest/fastobo_graphs/trait.FromGraph.html) and
  [`IntoGraph`](https://docs.rs/fastobo-graphs/latest/fastobo_graphs/trait.IntoGraph.html)
  traits that can be used to convert between a
  [`GraphDocument`](https://docs.rs/fastobo-graphs/latest/fastobo_graphs/model/struct.GraphDocument.html)
  and an [`OboDoc`](https://docs.rs/fastobo/latest/fastobo/ast/struct.OboDoc.html).


## Feedback

Found a bug ? Have an enhancement request ? Head over to the
[GitHub issue tracker](https://github.com/fastobo/fastobo-graphs/issues) of the project if
you need to report or ask something. If you are filling in on a bug, please include as much
information as you can about the issue, and try to recreate the same bug in a simple, easily
reproducible situation.


## About

This project was developed by [Martin Larralde](https://github.com/althonos)
as part of a Master's Degree internship in the [BBOP team](http://berkeleybop.org/) of the
[Lawrence Berkeley National Laboratory](https://www.lbl.gov/), under the supervision of
[Chris Mungall](http://biosciences.lbl.gov/profiles/chris-mungall/). Cite this project as:

*Larralde M.* **Developing Python and Rust libraries to improve the ontology ecosystem**
*\[version 1; not peer reviewed\].* F1000Research 2019, 8(ISCB Comm J):1500 (poster)
([https://doi.org/10.7490/f1000research.1117405.1](https://doi.org/10.7490/f1000research.1117405.1))
