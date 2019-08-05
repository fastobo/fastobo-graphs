use std::mem::replace;

use fastobo::ast::HeaderClause;
use fastobo::ast::EntityFrame;
use fastobo::ast::Ident;
use fastobo::ast::QuotedString;
use fastobo::ast::Synonym;
use fastobo::ast::OboDoc;
use fastobo::ast::SynonymScope;
use fastobo::ast::TermClause;
use fastobo::ast::TypedefClause;
use fastobo::ast::HeaderFrame;
use fastobo::ast::InstanceClause;
use fastobo::ast::TermFrame;
use fastobo::ast::IsoDateTime;
use fastobo::ast::InstanceFrame;
use fastobo::ast::TypedefFrame;
use fastobo::ast::Line;
use fastobo::ast::UnquotedString;
use fastobo::ast::Xref;
use fastobo::ast::XrefList;
use fastobo::ast::RelationIdent;
use fastobo::ast::PrefixedIdent;
use fastobo::ast::SubsetIdent;
use fastobo::ast::InstanceIdent;
use fastobo::ast::PropertyValue;
use fastobo::ast::Url;
use fastobo::semantics::Identified;
use fastobo::semantics::Orderable;
use fastobo::visit::VisitMut;
use fastobo::visit::IdDecompactor;

use crate::constants::property::dc;
use crate::constants::property::iao;
use crate::constants::property::obo_in_owl;
use crate::constants::property::rdfs;
use crate::error::Error;
use crate::error::Result;
use crate::model::GraphDocument;
use crate::model::Graph;
use crate::model::Meta;
use crate::model::Node;
use crate::model::NodeType;
use crate::model::BasicPropertyValue;
use crate::model::DefinitionPropertyValue;
use crate::model::SynonymPropertyValue;
use crate::model::XrefPropertyValue;
use super::Context;
use super::IntoGraph;
use super::IntoGraphCtx;

// FIXME: one graph per import, final = graph document ?
impl IntoGraphCtx<GraphDocument> for OboDoc {
    fn into_graph_ctx(mut self, ctx: &mut Context) -> Result<GraphDocument> {
        // Preprocess the document (macros, default namespace, urlize)
        // FIXME: the ID decompactor should make sure to preprocess the
        //        *shorthand* relationship.
        self.treat_xrefs();
        self.assign_namespaces();
        IdDecompactor::new().visit_doc(&mut self); // FIXME (for shorthand rel)?

        // Take ownership over the header and the entities.
        let header = replace(self.header_mut(), HeaderFrame::default());
        let entities = replace(self.entities_mut(), Vec::new());

        // Extract the graph ID using the `ontology` key
        // NB: `http://purl.obolibrary.org/obo/TEMP` is used as the default
        //     ontology IRI by the OWL API as well.
        let mut id = String::from("http://purl.obolibrary.org/obo/TEMP");
        for clause in header.iter() {
            if let HeaderClause::Ontology(s) = clause {
                id = format!("http://purl.obolibrary.org/obo/{}.owl", s);
            }
        }

        // Build the empty graph
        let mut graph = Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            id: ctx.ontology_iri.to_string(),
            label: None,
            meta: Box::new(Meta::from(header)),
            equivalent_nodes_sets: Vec::new(),
            logical_definition_axioms: Vec::new(),
            domain_range_axioms: Vec::new(),
            property_chain_axioms: Vec::new(),
        };

        // Extend the graph with all entities
        for entity in entities.into_iter() {
            // let mut entity_graph = entity.into_graph_ctx(ctx)?;
            // graph.extend(entity_graph);
            let mut entity_graph = entity.into();
            graph.extend(entity_graph);
        }

        // TODO: Add imports recursively
        // for clause in header.iter() {}
        Ok(GraphDocument::from(graph))
    }
}

impl IntoGraph for OboDoc {
    fn into_graph(self) -> Result<GraphDocument> {
        let mut ctx = Context::from(&self);
        self.into_graph_ctx(&mut ctx)
    }
}
