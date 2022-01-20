use std::collections::HashMap;

use fastobo::ast::HeaderClause;
use fastobo::ast::Ident;
use fastobo::ast::IdentPrefix;
use fastobo::ast::OboDoc;
use fastobo::ast::UnprefixedIdent;
use fastobo::ast::Url;

use super::constants::uri;
use super::error::Result;
use super::model::GraphDocument;

mod doc;
mod entity;
mod header;
mod pv;

pub struct Context {
    pub idspaces: HashMap<IdentPrefix, Url>,
    pub ontology_iri: Url,
    pub current_frame: Url,
    pub shorthands: HashMap<UnprefixedIdent, Ident>,
    // pub in_annotation: bool,
    // pub class_level: HashSet<Url>,
}

impl Context {
    /// Expand an identifier into the semantically-equivalent URI.
    pub fn expand<I: AsRef<Ident>>(&self, id: I) -> String {
        match id.as_ref() {
            Ident::Url(url) => url.to_string(),
            Ident::Prefixed(prf) => match self.idspaces.get(prf.prefix()) {
                Some(url) => format!("{}{}", url, prf.local()),
                None => format!("{}{}_{}", uri::OBO, prf.prefix(), prf.local()),
            },
            Ident::Unprefixed(unp) => match self.shorthands.get(unp) {
                Some(id) => self.expand(id),
                None => format!("{}#{}", self.ontology_iri, unp),
            },
        }
    }
}

impl From<&OboDoc> for Context {
    fn from(doc: &OboDoc) -> Self {
        // Add the ID spaces declared implicitly in the document.
        let mut idspaces = HashMap::new();
        idspaces.insert(
            IdentPrefix::new("BFO"),
            Url::new(format!("{}BFO_", uri::OBO,)).unwrap(),
        );
        idspaces.insert(
            IdentPrefix::new("RO"),
            Url::new(format!("{}RO", uri::OBO,)).unwrap(),
        );
        idspaces.insert(IdentPrefix::new("xsd"), Url::new(uri::XSD).unwrap());

        // Add the prefixes and ID spaces from the OBO header.
        let mut ontology_iri = Url::new("http://purl.obolibrary.org/obo/TEMP").unwrap();
        for clause in doc.header() {
            match clause {
                HeaderClause::Idspace(prefix, url, _) => {
                    idspaces.insert(prefix.as_ref().clone(), url.as_ref().clone());
                }
                HeaderClause::Ontology(slug) => {
                    ontology_iri = Url::new(format!("{}{}.owl", uri::OBO, slug)).unwrap();
                }
                _ => (),
            }
        }

        // Create the conversion context (FIXME: remove the unwraps ?).
        let shorthands = HashMap::new();
        let current_frame = ontology_iri.clone();
        Context {
            idspaces,
            ontology_iri,
            current_frame,
            shorthands,
        }
    }
}

/// Trait to convert an OBO syntax node into an OBO graph element.
trait IntoGraphCtx<T> {
    fn into_graph_ctx(self, ctx: &mut Context) -> Result<T>;
}

/// Trait to convert an OBO document into a complete OBO graph document.
#[cfg_attr(feature = "_doc", doc(cfg(feature = "obo")))]
pub trait IntoGraph {
    fn into_graph(self) -> Result<GraphDocument>;
}
