extern crate fastobo_graphs;
extern crate serde_json;
extern crate serde_yaml;

use std::fs::File;
use std::path::PathBuf;

pub mod parse {
    use super::*;

    macro_rules! test_impl {
        ($case:ident) => {
            #[test]
            #[allow(non_snake_case)]
            fn $case() {
                let basename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("obographs")
                    .join("examples")
                    .join(stringify!($case));
                let yamlfile = File::open(basename.with_extension("yaml")).unwrap();
                let jsonfile = File::open(basename.with_extension("json")).unwrap();
                let jsongraph: fastobo_graphs::model::GraphDocument =
                    serde_json::from_reader(jsonfile).unwrap();
                let yamlgraph: fastobo_graphs::model::GraphDocument =
                    serde_yaml::from_reader(yamlfile).unwrap();
                assert_eq!(jsongraph, yamlgraph, "graphs do not match!")
            }
        };
    }

    test_impl!(abox);
    test_impl!(basic);
    test_impl!(equivNodeSetTest);
    test_impl!(logicalDefinitionTest);
    test_impl!(nucleus);
    test_impl!(obsoletion_example);
    // test_impl!(ro);
}

pub mod convert {
    use super::*;
    use fastobo_graphs::FromGraph;

    macro_rules! test_impl {
        ($case:ident) => {
            #[test]
            #[allow(non_snake_case)]
            fn $case() {
                let yaml = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("obographs")
                    .join("examples")
                    .join(stringify!($case))
                    .with_extension("yaml");
                let yamlfile = File::open(yaml).unwrap();
                let yamlgraphs: fastobo_graphs::model::GraphDocument =
                    serde_yaml::from_reader(yamlfile).unwrap();
                let graph = yamlgraphs.graphs.into_iter().next().unwrap();

                if let Err(e) = fastobo::ast::OboDoc::from_graph(graph) {
                    panic!("could not convert {} to OBO: {}", stringify!($case), e);
                }
            }
        };
    }

    test_impl!(abox);
    test_impl!(basic);
    test_impl!(equivNodeSetTest);
    test_impl!(logicalDefinitionTest);
    test_impl!(nucleus);
    test_impl!(obsoletion_example);
    // test_impl!(ro);
}
