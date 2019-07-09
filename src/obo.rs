use std::str::FromStr;
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

use super::Graph;
use super::Meta;
use super::Node;
use super::NodeType;
use super::BasicPropertyValue;
use super::DefinitionPropertyValue;
use super::SynonymPropertyValue;
use super::XrefPropertyValue;

// ---------------------------------------------------------------------------

impl From<Graph> for OboDoc {
    fn from(graph: Graph) -> Self {

        let mut entities = HashMap::new();
        for frame in graph.nodes.into_iter().filter_map(<Option<EntityFrame>>::from) {
            entities.insert(frame.as_id().clone(), frame);
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
            for node in &eq.node_ids {
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

        let mut header = (*graph.meta).into();
        let mut entities = entities.into_iter().map(|(_, v)| v).collect();

        let mut doc = OboDoc::with_header(header).and_entities(entities);
        doc.sort();
        doc
    }
}

// ---------------------------------------------------------------------------

impl From<Meta> for HeaderFrame {
    fn from(meta: Meta) -> Self {
        let mut frame = Self::new();


        // FIXME
        frame
    }
}

impl From<BasicPropertyValue> for HeaderClause {
    fn from(pv: BasicPropertyValue) -> Self {
        match pv.pred.as_str() {
            "http://www.geneontology.org/formats/oboInOwl#hasOBOFormatVersion" => {
                HeaderClause::FormatVersion(UnquotedString::new(pv.val))
            }
            other => {
                let rel = RelationIdent::from_str(&other).expect("invalid relation ident");
                let pv = match Ident::from_str(&pv.val) {
                    Ok(id) => PropertyValue::Resource(rel, id),
                    Err(_) => PropertyValue::Literal(
                        rel,
                        QuotedString::new(pv.val),
                        Ident::from(PrefixedIdent::new("xsd", "string"))
                    )
                };
                HeaderClause::PropertyValue(pv)
            },
        }
    }
}

// ---------------------------------------------------------------------------

macro_rules! impl_frame_inner {
    ($node:expr, $id: expr, $ident: ident, $variant: ident) => {{
        mashup! {
            m[Variant] = $variant;
            m[Frame] = $variant Frame;
            m[Clause] = $variant Clause;
        }
        m! {
            let mut frame = Frame::new(Line::from($ident::from($id)));
            if let Some(label) = $node.label {
                let name = Clause::Name(UnquotedString::new(label));
                frame.push(Line::from(name));
            }
            if let Some(meta) = $node.meta {
                let clauses: Vec<Clause> = (*meta).into();
                frame.extend(clauses.into_iter().map(Line::from));
            }
            Some(EntityFrame::Variant(frame))
        }
    }}
}

impl From<Node> for Option<EntityFrame> {
    fn from(node: Node) -> Self {
        let id = Ident::from_str(&node.id).expect("invalid node id");
        match node.ty {
            None => None,
            Some(NodeType::Class) => {
                impl_frame_inner!(node, id, ClassIdent, Term)
            }
            Some(NodeType::Individual) => {
                impl_frame_inner!(node, id, InstanceIdent, Instance)
            }
            Some(NodeType::Property) => {
                impl_frame_inner!(node, id, RelationIdent, Typedef)
            }
        }
    }
}

// ---------------------------------------------------------------------------

macro_rules! impl_meta {
    ($clause:ident) => {
        impl From<Meta> for Vec<$clause> {
            fn from(meta: Meta) -> Self {
                let mut clauses = Vec::new();
                if let Some(desc) = meta.definition {
                    clauses.push((*desc).into())
                }
                for comment in meta.comments {
                    clauses.push($clause::Comment(UnquotedString::from(comment)));
                }
                for subset in meta.subsets {
                    let id = SubsetIdent::from_str(&subset).expect("invalid subset ident");
                    clauses.push($clause::Subset(id));
                }
                for xref in meta.xrefs {
                    clauses.push($clause::Xref(Xref::from(xref)));
                }
                for synonym in meta.synonyms {
                    clauses.push($clause::Synonym(Synonym::from(synonym)));
                }
                for pv in meta.basic_property_values {
                    clauses.push($clause::from(pv));
                }
                // FIXME: what to do with `meta.version` ?
                if meta.deprecated {
                    clauses.push($clause::IsObsolete(true));
                }
                clauses
            }
        }
    }
}

impl_meta!(TermClause);
impl_meta!(TypedefClause);
impl_meta!(InstanceClause);

// ---------------------------------------------------------------------------

macro_rules! impl_definition_pv {
    ($clause:ident) => {
        impl From<DefinitionPropertyValue> for $clause {
            fn from(pv: DefinitionPropertyValue) -> Self {
                let value = QuotedString::new(pv.val);
                let xrefs = pv.xrefs
                    .into_iter()
                    .map(|id: String| Ident::from_str(&id).map(Xref::new))
                    .collect::<Result<XrefList, _>>()
                    .expect("invalid xref id");
                $clause::Def(value, xrefs)
            }
        }
    }
}

impl_definition_pv!(TermClause);
impl_definition_pv!(TypedefClause);
impl_definition_pv!(InstanceClause);

// ---------------------------------------------------------------------------

macro_rules! impl_basic_pv_common {
    ($pv:ident, $clause:ident, $x:ident $(, $l:pat => $r:expr )* ) => {{
        match $x {
            "http://purl.org/dc/elements/1.1/creator" => {
                $clause::CreatedBy(UnquotedString::new($pv.val))
            },
            "http://www.w3.org/2000/01/rdf-schema#comment" => {
                $clause::Comment(UnquotedString::new($pv.val))
            },
            "http://www.geneontology.org/formats/oboInOwl#hasAlternativeId" => {
                let id = Ident::from_str(&$pv.val).expect("invalid ident");
                $clause::AltId(id.into())
            },
            "http://www.geneontology.org/formats/oboInOwl#hasOBONamespace" => {
                let id = Ident::from_str(&$pv.val).expect("invalid ident");
                $clause::Namespace(id.into())
            },
            "http://purl.org/dc/elements/1.1/date" => {
                let date = IsoDateTime::from_str(&$pv.val).expect("invalid date");
                $clause::CreationDate(date)
            },
            "http://purl.org/dc/elements/1.1/creator" => {
                $clause::CreatedBy(UnquotedString::new($pv.val))
            },
            "http://purl.obolibrary.org/obo/IAO_0100001" => {
                if let Ok(id) = Ident::from_str(&$pv.val) {
                    $clause::ReplacedBy(id.into())
                } else {
                    panic!("invalid ident (FIXME ?)")
                }
            }
            $( $l => $r ),*
            other => {
                let rel = RelationIdent::from_str(&other).expect("invalid relation ident");
                let pv = match Ident::from_str(&$pv.val) {
                    Ok(id) => PropertyValue::Resource(rel, id),
                    Err(_) => PropertyValue::Literal(
                        rel,
                        QuotedString::new($pv.val),
                        Ident::from(PrefixedIdent::new("xsd", "string"))
                    )
                };
                $clause::PropertyValue(pv)
            },
        }
    }};
}

impl From<BasicPropertyValue> for TermClause {
    fn from(pv: BasicPropertyValue) -> Self {
        let s = pv.pred.as_str();
        impl_basic_pv_common!(pv, TermClause, s)
    }
}

impl From<BasicPropertyValue> for TypedefClause {
    fn from(pv: BasicPropertyValue) -> Self {
        let s = pv.pred.as_str();
        impl_basic_pv_common!(pv, TypedefClause, s)
    }
}

impl From<BasicPropertyValue> for InstanceClause {
    fn from(pv: BasicPropertyValue) -> Self {
        let s = pv.pred.as_str();
        impl_basic_pv_common!(pv, InstanceClause, s)
    }
}

// ---------------------------------------------------------------------------

impl From<XrefPropertyValue> for Xref {
    fn from(pv: XrefPropertyValue) -> Self {
        // FIXME: what to do with label ? what to do with meta ?
        let id = Ident::from_str(&pv.val).expect("invalid xref ident");
        Xref::new(id)
    }
}

// ---------------------------------------------------------------------------

impl From<SynonymPropertyValue> for Synonym {
    fn from(pv: SynonymPropertyValue) -> Self {
        let desc = QuotedString::new(pv.val);
        let scope = match pv.pred.as_str() {
            "hasBroadSynonym" => SynonymScope::Broad,
            "hasExactSynonym" => SynonymScope::Exact,
            "hasNarrowSynonym" => SynonymScope::Narrow,
            "hasRelatedSynonym" => SynonymScope::Related,
            other => panic!("unknown synonym type: {}", other),
        };
        let xrefs = pv.xrefs
            .into_iter()
            .map(|id| Xref::new(Ident::from_str(&id).expect("invalid xref ident")))
            .collect::<XrefList>();
        Synonym::with_xrefs(desc, scope, xrefs)
    }
}
