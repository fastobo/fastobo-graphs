extern crate fastobo_graphs;
extern crate serde_json;
extern crate serde_yaml;

use std::io::BufRead;
use std::io::BufReader;

use fastobo_graphs::model::GraphDocument;

lazy_static::lazy_static! {
    /// The latest OBO Foundry listing.
    static ref FOUNDRY: obofoundry::Foundry = {
        let response = ureq::get("http://www.obofoundry.org/registry/ontologies.yml")
            .call()
            .unwrap();
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
            let res = ureq::builder()
                .redirects(10)
                .build()
                .get(url.as_str())
                .call()
                .unwrap();

            // parse the OBO file if it is a correct OBO file.
            let mut buf = BufReader::new(res.into_reader());
            let peek = buf.fill_buf().expect("could not read response");

            if peek.starts_with(b"{") {
                if let Err(e) = serde_json::from_reader::<_, GraphDocument>(&mut buf) {
                    panic!("{}", e)
                }
            } else {
                panic!("could not connect to `{}`: {}", url, std::str::from_utf8(peek).unwrap());
            }
        }
    )
}

foundrytest!(
    #[ignore]
    go(go, "go.json")
);
foundrytest!(
    #[ignore]
    go_basic(go, "go/go-basic.json")
);
foundrytest!(
    #[ignore]
    go_plus(go, "go/extensions/go-plus.json")
);
foundrytest!(pato(pato, "pato.json"));
foundrytest!(
    #[ignore]
    mondo(mondo, "mondo.json")
);
foundrytest!(
    #[ignore]
    zeco(zeco, "zeco.json")
);
