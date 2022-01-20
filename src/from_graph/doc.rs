use std::collections::HashMap;
use std::str::FromStr;

use fastobo::ast::ClassIdent;
use fastobo::ast::EntityFrame;
use fastobo::ast::HeaderClause;
use fastobo::ast::HeaderFrame;
use fastobo::ast::Ident;
use fastobo::ast::InstanceClause;
use fastobo::ast::Line;
use fastobo::ast::OboDoc;
use fastobo::ast::RelationIdent;
use fastobo::ast::TermClause;
use fastobo::ast::TypedefClause;

use fastobo::semantics::Identified;
use fastobo::semantics::Orderable;
use fastobo::visit::IdCompactor;
use fastobo::visit::VisitMut;

use super::FromGraph;
use crate::error::Error;
use crate::error::Result;
use crate::model::Graph;

impl FromGraph<Graph> for OboDoc {
    fn from_graph(graph: Graph) -> Result<Self> {
        let mut entities = HashMap::new();
        for node in graph.nodes.into_iter() {
            if let Some(frame) = <Option<EntityFrame>>::from_graph(node)? {
                entities.insert(frame.as_id().clone(), frame);
            }
        }

        for edge in graph.edges.iter() {
            let id_sub = Ident::from_str(&edge.sub)?;
            let id_pred = RelationIdent::from_str(&edge.pred)?;
            let id_obj = Ident::from_str(&edge.obj)?;
            if &edge.pred == "is_a" || &edge.pred == "subPropertyOf" || &edge.pred == "subClassOf" {
                match entities.get_mut(&id_sub) {
                    Some(EntityFrame::Term(ref mut frame)) => {
                        let c = TermClause::IsA(Box::new(From::from(id_obj)));
                        frame.push(Line::from(c));
                    }
                    Some(EntityFrame::Typedef(ref mut frame)) => {
                        let c = TypedefClause::IsA(Box::new(From::from(id_obj)));
                        frame.push(Line::from(c));
                    }
                    Some(EntityFrame::Instance(_)) => {
                        return Err(Error::invalid_instance_clause("is_a"));
                    }
                    None => (),
                }
            } else if &edge.pred == "inverseOf" {
                match entities.get_mut(&id_sub) {
                    Some(EntityFrame::Typedef(ref mut frame)) => {
                        let c = TypedefClause::InverseOf(Box::new(From::from(id_obj)));
                        frame.push(Line::from(c));
                    }
                    Some(EntityFrame::Term(ref mut frame)) => {
                        return Err(Error::invalid_term_clause("inverse_of"));
                    }
                    Some(EntityFrame::Instance(_)) => {
                        return Err(Error::invalid_instance_clause("inverse_of"));
                    }
                    None => (),
                }
            } else {
                match entities.get_mut(&id_sub) {
                    Some(EntityFrame::Term(ref mut frame)) => {
                        let c = TermClause::Relationship(
                            Box::new(id_pred),
                            Box::new(From::from(id_obj)),
                        );
                        frame.push(Line::from(c));
                    }
                    Some(EntityFrame::Typedef(ref mut frame)) => {
                        let c = TypedefClause::Relationship(
                            Box::new(id_pred),
                            Box::new(From::from(id_obj)),
                        );
                        frame.push(Line::from(c));
                    }
                    Some(EntityFrame::Instance(ref mut frame)) => {
                        let c = InstanceClause::Relationship(
                            Box::new(id_pred),
                            Box::new(From::from(id_obj)),
                        );
                        frame.push(Line::from(c));
                    }
                    None => (),
                }
            }
        }

        for eq in graph.equivalent_nodes_sets.iter() {
            for node in eq.node_ids.iter() {
                let node_id = Ident::from_str(&node)?;
                match entities.get_mut(&node_id) {
                    Some(EntityFrame::Term(ref mut frame)) => {
                        for node in eq.node_ids.iter().filter(|&n| n != node) {
                            let id = ClassIdent::from_str(&node).map(Box::new)?;
                            frame.push(Line::from(TermClause::EquivalentTo(id)));
                        }
                    }
                    Some(EntityFrame::Typedef(ref mut frame)) => {
                        for node in eq.node_ids.iter().filter(|&n| n != node) {
                            let id = RelationIdent::from_str(&node).map(Box::new)?;
                            frame.push(Line::from(TypedefClause::EquivalentTo(id)));
                        }
                    }
                    Some(EntityFrame::Instance(_)) => {
                        return Err(Error::invalid_instance_clause("equivalent_to"));
                    }
                    None => (),
                }
            }
        }

        for dr in graph.domain_range_axioms.iter() {
            let id = Ident::from_str(&dr.predicate_id)?;
            if let Some(EntityFrame::Typedef(ref mut frame)) = entities.get_mut(&id) {
                for domain in dr.domain_class_ids.iter() {
                    let domain_id = ClassIdent::from_str(&domain).map(Box::new)?;
                    frame.push(Line::from(TypedefClause::Domain(domain_id)));
                }
                for range in dr.range_class_ids.iter() {
                    let range_id = ClassIdent::from_str(&range).map(Box::new)?;
                    frame.push(Line::from(TypedefClause::Range(range_id)));
                }
                // TODO: allValuesFromEdges
            }
        }

        // extract header
        let version_iri = graph.meta.version.clone();
        let mut header = HeaderFrame::from_graph(*graph.meta)?;
        if let Some(ont) = graph.id.strip_prefix(crate::constants::uri::OBO) {
            let id = ont.trim_end_matches(".obo").trim_end_matches(".owl");
            header.push(HeaderClause::Ontology(Box::new(id.into())));
            if let Some(ref v) = version_iri
                .as_ref()
                .and_then(|url| url.strip_prefix(crate::constants::uri::OBO))
            {
                let version = v
                    .trim_start_matches(id)
                    .trim_end_matches(".owl")
                    .trim_end_matches(".obo")
                    .trim_end_matches(id)
                    .trim_matches('/');
                header.push(HeaderClause::DataVersion(Box::new(version.into())))
            }
        }

        // collect entities and merge them into a document
        let mut doc = OboDoc::with_header(header)
            .and_entities(entities.into_iter().map(|(_, v)| v).collect());
        doc.sort();
        IdCompactor::new().visit_doc(&mut doc);

        Ok(doc)
    }
}
