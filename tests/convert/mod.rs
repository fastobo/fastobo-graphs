extern crate fastobo_graphs;
extern crate serde_json;
extern crate serde_yaml;

use std::fs::File;
use std::path::PathBuf;

use fastobo_graphs::FromGraph;
use fastobo_graphs::model::Graph;
use fastobo_graphs::model::GraphDocument;

pub mod into_graph {
    use super::*;

    macro_rules! test_impl {
        ($case:ident) => {
            #[test]
            #[allow(non_snake_case)]
            fn $case() {
                let basename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("tests")
                    .join("convert")
                    .join(stringify!($case));
                let obofile = basename.with_extension("obo");
                let jsonfile = File::open(basename.with_extension("json")).unwrap();

                let obo = fastobo::ast::OboDoc::from_file(obofile).unwrap();
                let expected: GraphDocument = serde_json::from_reader(jsonfile).unwrap();
                let actual = GraphDocument::from(Graph::from(obo));

                assert_eq!(expected, actual, "graphs do not match!")
            }
        }
    }

    test_impl!(header_saved_by);
}
