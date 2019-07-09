use fastobo::ast::HeaderClause;
use fastobo::ast::EntityFrame;
use fastobo::ast::Ident;
use fastobo::ast::QuotedString;
use fastobo::ast::Synonym;
use fastobo::ast::OboDoc;
use fastobo::ast::SynonymScope;
use fastobo::ast::TermClause;
use fastobo::ast::TypedefClause;
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

impl From<EntityFrame> for Graph {
    fn from(frame: EntityFrame) -> Self {
        match frame {
            EntityFrame::Term(t) => t.into(),
            EntityFrame::Typedef(t) => t.into(),
            EntityFrame::Instance(t) => t.into(),
        }
    }
}

impl From<TermFrame> for Graph {
    fn from(term: TermFrame) -> Self {
        use fastobo::ast::TermClause::*;

        //
        let mut meta = Meta::default();
        let mut node = Node {
            id: term.id().to_string(), // FIXME: urllize
            meta: None,
            ty: Some(NodeType::Class),
            label: None
        };



        for line in term.into_iter() {
            let clause = line.into_inner();
            match clause {
                IsAnonymous(val) => {

                }
                Name(name) => {

                }
                Namespace(ns) => {
                    meta.basic_property_values.push(
                        BasicPropertyValue::new(
                            obo_in_owl::HAS_OBO_NAMESPACE.to_string(),
                            ns.to_string(),
                        )
                    );
                }
                AltId(alt_id) => {}
                Def(def, xrefs) => {}
                Comment(comment) => {}
                Subset(subset) => {}
                Synonym(syn) => {}
                Xref(xref) => {}
                Builtin(bool) => {}
                PropertyValue(pv) => {}
                IsA(cid) => {}
                IntersectionOf(optrid, cid) => {}
                UnionOf(cid) => {}
                EquivalentTo(cid) => {}
                DisjointFrom(cid) => {}
                Relationship(rid, cid) => {}
                CreatedBy(name) => {}
                CreationDate(dt) => {}
                IsObsolete(val) => {}
                ReplacedBy(cid) => {}
                Consider(cid) => {}
            }
        }

        node.meta = Some(Box::new(meta));

        Graph {
            id: node.id.clone(),
            nodes: Some(node).into_iter().collect(),
            edges: Vec::new(),
            label: None,
            meta: Box::new(Meta::default()),
            equivalent_nodes_sets: Vec::new(),
            logical_definition_axioms: Vec::new(),
            domain_range_axioms: Vec::new(),
            property_chain_axioms: Vec::new(),
        }
    }
}

impl From<TypedefFrame> for Graph {
    fn from(typedef: TypedefFrame) -> Self {
        // ... TODO ... //
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            id: typedef.id().to_string(),
            label: None,
            meta: Box::new(Meta::default()),
            equivalent_nodes_sets: Vec::new(),
            logical_definition_axioms: Vec::new(),
            domain_range_axioms: Vec::new(),
            property_chain_axioms: Vec::new(),
        }
    }
}

impl From<InstanceFrame> for Graph {
    fn from(instance: InstanceFrame) -> Self {
        // ... TODO ... //
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            id: instance.id().to_string(),
            label: None,
            meta: Box::new(Meta::default()),
            equivalent_nodes_sets: Vec::new(),
            logical_definition_axioms: Vec::new(),
            domain_range_axioms: Vec::new(),
            property_chain_axioms: Vec::new(),
        }
    }
}
