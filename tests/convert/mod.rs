#[macro_use]
extern crate pretty_assertions;

extern crate fastobo_graphs;
extern crate serde_json;
extern crate serde_yaml;

use std::fs::File;
use std::path::PathBuf;

use fastobo_graphs::model::GraphDocument;
use fastobo_graphs::FromGraph;
use fastobo_graphs::IntoGraph;

macro_rules! test_impl {
    ($($name:ident,)*) => (
        pub mod into_graph {
            use super::*;

            macro_rules! test_impl {
                ($case:ident) => {
                    #[test]
                    #[ignore]
                    #[allow(non_snake_case)]
                    fn $case() {
                        let basename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                            .join("tests")
                            .join("convert")
                            .join(stringify!($case));
                        let obofile = basename.with_extension("obo");
                        let jsonfile = File::open(basename.with_extension("json")).unwrap();

                        let obo = fastobo::from_file(obofile).unwrap();
                        let expected: GraphDocument = serde_json::from_reader(jsonfile).unwrap();
                        let actual = obo.into_graph().unwrap();

                        assert_eq!(expected, actual, "graphs do not match!")
                    }
                }
            }

            $(test_impl!($name);)*
        }

        pub mod from_graph {
            use super::*;

            macro_rules! test_impl {
                ($case:ident) => {
                    #[test]
                    #[ignore]
                    #[allow(non_snake_case)]
                    fn $case() {
                        let basename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                            .join("tests")
                            .join("convert")
                            .join(stringify!($case));
                        let obofile = basename.with_extension("obo");
                        let jsonfile = File::open(basename.with_extension("json")).unwrap();

                        let expected = fastobo::from_file(obofile).unwrap();
                        let doc: GraphDocument = serde_json::from_reader(jsonfile).unwrap();
                        let graph = doc.graphs.into_iter().next().unwrap();
                        let actual = fastobo::ast::OboDoc::from_graph(graph).unwrap();

                        assert_eq!(expected, actual, "graphs do not match!")
                    }
                }
            }

            $(test_impl!($name);)*
        }
    )
}

test_impl!(
    header_data_version,
    header_date,
    header_default_namespace,
    header_format_version,
    header_namespace_id_rule,
    header_remark,
    header_saved_by,
    header_subsetdef,
    header_synonymtypedef,
);
