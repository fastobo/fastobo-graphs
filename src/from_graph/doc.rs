use std::mem::replace;
use std::str::FromStr;
use std::string::ToString;
use std::collections::HashMap;

use fastobo::ast::ClassIdent;
use fastobo::ast::HeaderFrame;
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

use super::FromGraph;
use crate::error::Error;
use crate::error::Result;

impl FromGraph<Graph> for OboDoc {
    fn from_graph(graph: Graph) -> Result<Self> {

        let mut entities = HashMap::new();
        for node in graph.nodes.into_iter() {
            if let Some(frame) = <Option<EntityFrame>>::from_graph(node)? {
                entities.insert(frame.as_id().clone(), frame);
            }
        }

        for edge in graph.edges.iter() {
            let id_sub = Ident::from_str(&edge.sub).expect("invalid ident");
            let id_pred = RelationIdent::from_str(&edge.pred).expect("invalid relation ident");
            let id_obj = Ident::from_str(&edge.obj).expect("invalid ident");
            if &edge.pred == "is_a" || &edge.pred == "subPropertyOf" || &edge.pred == "subClassOf" {
                match entities.get_mut(&id_sub) {
                    Some(EntityFrame::Term(ref mut frame)) => {
                        let c = TermClause::IsA(From::from(id_obj));
                        frame.push(Line::from(c));
                    }
                    Some(EntityFrame::Typedef(ref mut frame)) => {
                        let c = TypedefClause::IsA(From::from(id_obj));
                        frame.push(Line::from(c));
                    }
                    Some(EntityFrame::Instance(_)) => {
                        panic!("cannot have `is_a` on instance clause");
                    },
                    None => (),
                }
            } else if &edge.pred == "inverseOf" {
                match entities.get_mut(&id_sub) {
                    Some(EntityFrame::Typedef(ref mut frame)) => {
                        let c = TypedefClause::InverseOf(From::from(id_obj));
                        frame.push(Line::from(c));
                    }
                    Some(EntityFrame::Term(ref mut frame)) => {
                        panic!("cannot have `inverse_of` on term clause");
                    }
                    Some(EntityFrame::Instance(_)) => {
                        panic!("cannot have `inverse_of` on instance clause");
                    },
                    None => (),
                }
            } else {
                match entities.get_mut(&id_sub) {
                    Some(EntityFrame::Term(ref mut frame)) => {
                        let c = TermClause::Relationship(id_pred, From::from(id_obj));
                        frame.push(Line::from(c));
                    }
                    Some(EntityFrame::Typedef(ref mut frame)) => {
                        let c = TypedefClause::Relationship(id_pred, From::from(id_obj));
                        frame.push(Line::from(c));
                    }
                    Some(EntityFrame::Instance(ref mut frame)) => {
                        let c = InstanceClause::Relationship(id_pred, From::from(id_obj));
                        frame.push(Line::from(c));
                    }
                    None => (),
                }
            }
        }

        for eq in graph.equivalent_nodes_sets.iter() {
            for node in eq.node_ids.iter() {
                let node_id = Ident::from_str(&node).expect("invalid ident");
                match entities.get_mut(&node_id) {
                    Some(EntityFrame::Term(ref mut frame)) => {
                        for node in eq.node_ids.iter().filter(|&n| n != node) {
                            let id = ClassIdent::from_str(&node).expect("invalid ident");
                            frame.push(Line::from(TermClause::EquivalentTo(id)));
                        }
                    }
                    Some(EntityFrame::Typedef(ref mut frame)) => {
                        for node in eq.node_ids.iter().filter(|&n| n != node) {
                            let id = RelationIdent::from_str(&node).expect("invalid ident");
                            frame.push(Line::from(TypedefClause::EquivalentTo(id)));
                        }
                    }
                    Some(EntityFrame::Instance(_)) => {
                        panic!("cannot have `equivalent_to` on instance clause");
                    },
                    None => (),
                }
            }
        }

        for dr in graph.domain_range_axioms.iter() {
            let id = Ident::from_str(&dr.predicate_id).expect("invalid ident");
            if let Some(EntityFrame::Typedef(ref mut frame)) = entities.get_mut(&id) {
                for domain in dr.domain_class_ids.iter() {
                    let domain_id = ClassIdent::from_str(&domain).expect("invalid ident");
                    frame.push(Line::from(TypedefClause::Domain(domain_id)));
                }
                for range in dr.range_class_ids.iter() {
                    let range_id = ClassIdent::from_str(&range).expect("invalid ident");
                    frame.push(Line::from(TypedefClause::Range(range_id)));
                }
                // TODO: allValuesFromEdges
            }
        }

        let mut header = FromGraph::from_graph(*graph.meta)?;
        let mut entities = entities.into_iter().map(|(_, v)| v).collect();

        let mut doc = OboDoc::with_header(header).and_entities(entities);
        doc.sort();
        Ok(doc)
    }
}
