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
                    .join("tests")
                    .join("data")
                    .join(stringify!($case));
                let jsonfile = File::open(basename.with_extension("json")).unwrap();
                let _jsongraph: fastobo_graphs::model::GraphDocument =
                    serde_json::from_reader(jsonfile).unwrap();
            }
        };
    }

    test_impl!(issue20);
}
