extern crate fastobo;
extern crate fastobo_graphs;
extern crate serde_json;
extern crate serde_yaml;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use fastobo::visit::VisitMut;
use fastobo_graphs::model::GraphDocument;
use fastobo_graphs::FromGraph;

fn main() {
    for path in std::env::args().skip(1) {
        // Open the file
        let srcpath = PathBuf::from(&path);
        let dstpath = srcpath.with_extension("obo");
        let srcfile = File::open(&srcpath).unwrap();

        // Parse the file (using JSON or YAML parser depending on the
        // file extension of the input).
        let doc = match srcpath.extension() {
            Some(s) if s == "json" => {
                match serde_yaml::from_reader::<File, GraphDocument>(srcfile) {
                    Ok(doc) => doc,
                    Err(e) => panic!("{} could not be parsed:\n{}", path, e),
                }
            }
            Some(s) if s == "yaml" => {
                match serde_yaml::from_reader::<File, GraphDocument>(srcfile) {
                    Ok(doc) => doc,
                    Err(e) => panic!("{} could not be parsed:\n{}", path, e),
                }
            }
            Some(other) => panic!("unknown file extension: {:?}", other),
            None => panic!("can't determine input file type from extension"),
        };

        // Verify only one graph is present: we can't create an OBO document
        // with more than one graph.
        if doc.graphs.len() > 1 {
            panic!("input file contains more than one graph");
        }

        // Write the generated document to an OBO file next to the input file.
        let graph = doc.graphs.into_iter().next().unwrap();
        let mut obodoc =
            fastobo::ast::OboDoc::from_graph(graph).expect("could not convert from graph");
        fastobo::visit::IdCompactor::new().visit_doc(&mut obodoc);
        File::create(&dstpath)
            .and_then(|mut f| write!(f, "{}", obodoc))
            .expect("could not write output file");
    }
}
