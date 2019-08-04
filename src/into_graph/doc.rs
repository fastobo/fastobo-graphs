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

use crate::constants::property::dc;
use crate::constants::property::iao;
use crate::constants::property::obo_in_owl;
use crate::constants::property::rdfs;
use crate::model::Graph;
use crate::model::Meta;
use crate::model::Node;
use crate::model::NodeType;
use crate::model::BasicPropertyValue;
use crate::model::DefinitionPropertyValue;
use crate::model::SynonymPropertyValue;
use crate::model::XrefPropertyValue;

// FIXME: one graph per import, final = graph document ?
impl From<OboDoc> for Graph {
    fn from(mut doc: OboDoc) -> Self {
        let header = replace(doc.header_mut(), HeaderFrame::default());
        let entities = replace(doc.entities_mut(), Vec::new());

        // Extract the graph ID using the `ontology` key
        let mut id = String::from("http://purl.obolibrary.org/obo/TEMP");
        for clause in header.iter() {
            if let HeaderClause::Ontology(s) = clause {
                id = format!("http://purl.obolibrary.org/obo/{}.owl", s);
            }
        }

        // Build the empty graph
        let mut graph = Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            id,
            label: None,
            meta: Box::new(Meta::from(header)),
            equivalent_nodes_sets: Vec::new(),
            logical_definition_axioms: Vec::new(),
            domain_range_axioms: Vec::new(),
            property_chain_axioms: Vec::new(),
        };

        // Extend the graph with all entities
        for entity in entities.into_iter() {
            let mut entity_graph: Graph = entity.into();
            graph.extend(entity_graph);
        }

        graph
    }
}
