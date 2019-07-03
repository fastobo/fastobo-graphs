extern crate fastobo_graphs;
extern crate serde_json;
extern crate serde_yaml;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use fastobo_graphs::GraphDocument;

lazy_static::lazy_static! {
    /// The latest OBO Foundry listing.
    static ref FOUNDRY: obofoundry::Foundry = {
        let response = ureq::get("http://www.obofoundry.org/registry/ontologies.yml")
            .call();
        serde_yaml::from_reader(response.into_reader())
            .expect("could not read the OBO Foundry listing")
    };
}

macro_rules! foundrytest {
    ( $(#[$attr:meta])* $name:ident($ont:ident, $product:expr)) => (
        $(#[$attr])*
        #[test]
        fn $name() {
            // get the URL to the OBO product
            let ref url = FOUNDRY
                .ontologies
                .iter()
                .find(|onto| onto.id == stringify!($ont))
                .expect("could not find ontology")
                .products
                .iter()
                .find(|prod| prod.id == $product)
                .expect("could not find json product")
                .ontology_purl;

            // get the OBO document
            let res = ureq::get(url.as_str())
                .redirects(10)
                .call();

            // parse the OBO file if it is a correct OBO file.
            let mut buf = BufReader::new(res.into_reader());
            let peek = buf.fill_buf().expect("could not read response");

            if peek.starts_with(b"{") {
                match serde_json::from_reader::<_, GraphDocument>(&mut buf) {
                    Ok(doc) => println!("ok"),
                    Err(e) => panic!("{}", e),
                }
            } else {
                panic!("could not connect to `{}`: {}", url, std::str::from_utf8(peek).unwrap());
            }
        }
    )
}

foundrytest!(go(go, "go.json"));
foundrytest!(go_basic(go, "go/go-basic.json"));
foundrytest!(go_plus(go, "go/extensions/go-plus.json"));

foundrytest!(pato(pato, "pato.json"));

foundrytest!(mondo(mondo, "mondo.json"));
foundrytest!(mondo_minimal(mondo, "mondo-minimal.json"));
