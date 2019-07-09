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

impl From<OboDoc> for Graph {
    fn from(mut doc: OboDoc) -> Self {

        let header = replace(doc.header_mut(), HeaderFrame::default());
        let entities = replace(doc.entities_mut(), Vec::new());

        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut equivalent_nodes_sets = Vec::new();
        let mut logical_definition_axioms = Vec::new();
        let mut domain_range_axioms = Vec::new();
        let mut property_chain_axioms = Vec::new();

        Self {
            nodes,
            edges,
            id: String::from("NONE"),
            label: None,
            meta: Box::new(Meta::from(header)),
            equivalent_nodes_sets: Vec::new(),
            logical_definition_axioms: Vec::new(),
            domain_range_axioms: Vec::new(),
            property_chain_axioms: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------

impl From<Meta> for HeaderFrame {
    fn from(meta: Meta) -> Self {
        let mut frame = Self::new();
        // ... TODO ... //
        frame
    }
}

impl From<HeaderFrame> for Meta {
    fn from(frame: HeaderFrame) -> Self {
        use fastobo::ast::HeaderClause::*;

        let mut definition = None;
        let mut comments = Vec::new();
        let mut subsets = Vec::new();
        let mut xrefs = Vec::new();
        let mut synonyms = Vec::new();
        let mut basic_property_values = Vec::new();
        let mut version = None;
        let mut deprecated = false;

        for clause in frame.into_iter() {
            match clause {
                FormatVersion(v) => {
                    basic_property_values.push(
                        BasicPropertyValue::new(
                            obo_in_owl::HAS_OBO_FORMAT_VERSION.to_string(),
                            v.into_string(),
                        )
                    );
                },
                DataVersion(v) => {
                    // FIXME: use OBO URL instead of OBO short name.
                    version = Some(v.into_string());
                },
                Date(dt) => {
                    basic_property_values.push(
                        BasicPropertyValue::new(
                            obo_in_owl::HAS_DATE.to_string(),
                            dt.to_string(),
                        )
                    );
                },
                SavedBy(name) => {
                    basic_property_values.push(
                        BasicPropertyValue::new(
                            obo_in_owl::SAVED_BY.to_string(),
                            name.into_string(),
                        )
                    );
                },
                AutoGeneratedBy(name) => {
                    basic_property_values.push(
                        BasicPropertyValue::new(
                            obo_in_owl::AUTO_GENERATED_BY.to_string(),
                            name.into_string(),
                        )
                    );
                },
                Import(import) => (),
                Subsetdef(id, def) => {
                    subsets.push(id.to_string());
                },
                SynonymTypedef(ty, def, optscope) => (),
                DefaultNamespace(ns) => {
                    basic_property_values.push(
                        BasicPropertyValue::new(
                            obo_in_owl::HAS_DEFAULT_NAMESPACE.to_string(),
                            ns.to_string(),
                        )
                    );
                },
                NamespaceIdRule(idrule) => {
                    basic_property_values.push(
                        BasicPropertyValue::new(
                            obo_in_owl::NAMESPACE_ID_RULE.to_string(),
                            idrule.into_string(),
                        )
                    );
                },
                Idspace(prefix, url, optdef) => (),
                TreatXrefsAsEquivalent(prefix) => (),
                TreatXrefsAsGenusDifferentia(prefix, rid, cid) => (),
                TreatXrefsAsReverseGenusDifferentia(prefix, rid, cid) => (),
                TreatXrefsAsRelationship(prefix, rid) => (),
                TreatXrefsAsIsA(prefix) => (),
                TreatXrefsAsHasSubclass(prefix) => (),
                PropertyValue(pv) => {
                    basic_property_values.push(BasicPropertyValue::from(pv));
                },
                Remark(remark) => {
                    comments.push(remark.into_string());
                },
                Ontology(ontology) => (),
                OwlAxioms(axioms) => (),
                Unreserved(key, value) => (),
            }
        }

        Self {
            definition,
            comments,
            subsets,
            xrefs,
            synonyms,
            basic_property_values,
            version,
            deprecated,
        }
    }
}

impl From<BasicPropertyValue> for HeaderClause {
    fn from(pv: BasicPropertyValue) -> Self {
        match pv.pred.as_str() {
            obo_in_owl::HAS_OBO_FORMAT_VERSION => {
                HeaderClause::FormatVersion(UnquotedString::new(pv.val))
            }
            // ...TODO... //
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
            dc::CREATOR => {
                $clause::CreatedBy(UnquotedString::new($pv.val))
            },
            rdfs::COMMENT => {
                $clause::Comment(UnquotedString::new($pv.val))
            },
            obo_in_owl::HAS_ALTERNATIVE_ID => {
                let id = Ident::from_str(&$pv.val).expect("invalid ident");
                $clause::AltId(id.into())
            },
            obo_in_owl::HAS_OBO_NAMESPACE => {
                let id = Ident::from_str(&$pv.val).expect("invalid ident");
                $clause::Namespace(id.into())
            },
            dc::DATE => {
                let date = IsoDateTime::from_str(&$pv.val).expect("invalid date");
                $clause::CreationDate(date)
            },
            iao::REPLACED_BY => {
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

// ---------------------------------------------------------------------------

impl From<PropertyValue> for BasicPropertyValue {
    fn from(pv: PropertyValue) -> Self {
        match pv {
            PropertyValue::Resource(rel, id) => {
                BasicPropertyValue::new(rel.to_string(), id.to_string())
            }
            PropertyValue::Literal(rel, value, ty) => {
                BasicPropertyValue::new(rel.to_string(), value.into_string())
            }
        }
    }
}
