use std::collections::HashMap;
use std::collections::HashSet;

use fastobo::ast::IdentPrefix;
use fastobo::ast::OboDoc;
use fastobo::ast::HeaderClause;
use fastobo::ast::Url;

use super::error::Error;
use super::error::Result;
use super::model::GraphDocument;
use super::constants::uri;

mod doc;
mod entity;
mod header;
mod pv;

pub struct Context {
    pub idspaces: HashMap<IdentPrefix, Url>,
    pub ontology_iri: Url,
    pub current_frame: Url,

    // pub in_annotation: bool,
    // pub class_level: HashSet<Url>,
}

impl From<&OboDoc> for Context {
    fn from(doc: &OboDoc) -> Self {
        /// Add the ID spaces declared implicitly in the document.
        let mut idspaces = HashMap::new();
        idspaces.insert(
            IdentPrefix::new("BFO"),
            Url::parse(&format!("{}BFO_", uri::OBO,)).unwrap(),
        );
        idspaces.insert(
            IdentPrefix::new("RO"),
            Url::parse(&format!("{}RO", uri::OBO,)).unwrap(),
        );
        idspaces.insert(
            IdentPrefix::new("xsd"),
            Url::parse(uri::XSD).unwrap(),
        );

        // Add the prefixes and ID spaces from the OBO header.
        let mut ontology = None;
        for clause in doc.header() {
            match clause {
                HeaderClause::Idspace(prefix, url, _) => {
                    idspaces.insert(prefix.clone(), url.clone());
                }
                HeaderClause::Ontology(id) => {
                    ontology = Some(id.to_string());
                }
                _ => (),
            }
        }

        // Create the conversion context (FIXME: remove the unwraps ?).
        let ontology_iri = Url::parse(&format!("{}{}", uri::OBO, ontology.unwrap())).unwrap();
        let current_frame = ontology_iri.clone();
        Context {
            idspaces,
            ontology_iri,
            current_frame
        }
    }
}

pub trait IntoGraphCtx<T> {
    fn into_graph_ctx(self, ctx: &mut Context) -> Result<T>;
}

pub trait IntoGraph {
    fn into_graph(self) -> Result<GraphDocument>;
}
