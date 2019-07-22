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
use crate::error::Result;
use crate::error::Error;
use crate::model::Graph;
use crate::model::Meta;
use crate::model::Node;
use crate::model::NodeType;
use crate::model::BasicPropertyValue;
use crate::model::DefinitionPropertyValue;
use crate::model::SynonymPropertyValue;
use crate::model::XrefPropertyValue;
use super::FromGraph;

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
                let clauses: Vec<Clause> = FromGraph::from_graph(*meta)?;
                frame.extend(clauses.into_iter().map(Line::from));
            }
            Ok(Some(EntityFrame::Variant(frame)))
        }
    }}
}

impl FromGraph<Node> for Option<EntityFrame> {
    fn from_graph(node: Node) -> Result<Self> {
        let id = Ident::from_str(&node.id)?;
        match node.ty {
            None => Ok(None),
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
        impl FromGraph<Meta> for Vec<$clause> {
            fn from_graph(meta: Meta) -> Result<Self> {
                let mut clauses = Vec::new();
                if let Some(desc) = meta.definition {
                    clauses.push($clause::from_graph(*desc)?)
                }
                for comment in meta.comments {
                    clauses.push($clause::Comment(UnquotedString::from(comment)));
                }
                for subset in meta.subsets {
                    let id = SubsetIdent::from_str(&subset)?;
                    clauses.push($clause::Subset(id));
                }
                for xref in meta.xrefs {
                    clauses.push($clause::Xref(Xref::from_graph(xref)?));
                }
                for synonym in meta.synonyms {
                    clauses.push($clause::Synonym(Synonym::from_graph(synonym)?));
                }
                for pv in meta.basic_property_values {
                    clauses.push($clause::from_graph(pv)?);
                }
                if meta.deprecated {
                    clauses.push($clause::IsObsolete(true));
                }
                Ok(clauses)
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
        impl FromGraph<DefinitionPropertyValue> for $clause {
            fn from_graph(pv: DefinitionPropertyValue) -> Result<Self> {
                let value = QuotedString::new(pv.val);
                let xrefs = pv.xrefs
                    .into_iter()
                    .map(|id: String| Ident::from_str(&id).map(Xref::new).map_err(Error::from))
                    .collect::<Result<XrefList>>()?;
                Ok($clause::Def(value, xrefs))
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
                Ok($clause::CreatedBy(UnquotedString::new($pv.val)))
            },
            rdfs::COMMENT => {
                Ok($clause::Comment(UnquotedString::new($pv.val)))
            },
            obo_in_owl::HAS_ALTERNATIVE_ID => {
                let id = Ident::from_str(&$pv.val)?;
                Ok($clause::AltId(id.into()))
            },
            obo_in_owl::HAS_OBO_NAMESPACE => {
                let id = Ident::from_str(&$pv.val)?;
                Ok($clause::Namespace(id.into()))
            },
            obo_in_owl::CREATED_BY | dc::CREATOR => {
                Ok($clause::CreatedBy(UnquotedString::new($pv.val)))
            }
            obo_in_owl::CREATION_DATE | dc::DATE => {
                let dt = IsoDateTime::from_str(&$pv.val)?;
                Ok($clause::CreationDate(dt))
            }
            iao::REPLACED_BY => {
                let id = Ident::from_str(&$pv.val)?;
                Ok($clause::ReplacedBy(id.into()))
            }
            $( $l => $r ),*
            other => {
                let rel = RelationIdent::from_str(&other)?;
                let pv = match Ident::from_str(&$pv.val) {
                    Ok(id) => PropertyValue::Resource(rel, id),
                    Err(_) => PropertyValue::Literal(
                        rel,
                        QuotedString::new($pv.val),
                        Ident::from(PrefixedIdent::new("xsd", "string"))
                    )
                };
                Ok($clause::PropertyValue(pv))
            },
        }
    }};
}

impl FromGraph<BasicPropertyValue> for TermClause {
    fn from_graph(pv: BasicPropertyValue) -> Result<Self> {
        let s = pv.pred.as_str();
        impl_basic_pv_common!(pv, TermClause, s)
    }
}

impl FromGraph<BasicPropertyValue>  for TypedefClause {
    fn from_graph(pv: BasicPropertyValue) -> Result<Self> {
        let s = pv.pred.as_str();
        impl_basic_pv_common!(pv, TypedefClause, s,
            obo_in_owl::IS_CYCLIC => {
                match bool::from_str(&pv.val) {
                    Ok(b) => Ok(TypedefClause::IsCyclic(b)),
                    Err(e) => Err(Error::InvalidBoolean(e, pv.val.to_string())),
                }
            },
            iao::ANTISYMMETRIC_PROPERTY => {
                match bool::from_str(&pv.val) {
                    Ok(b) => Ok(TypedefClause::IsAntiSymmetric(b)),
                    Err(e) => Err(Error::InvalidBoolean(e, pv.val.to_string())),
                }
            },
            obo_in_owl::IS_CLASS_LEVEL => {
                match bool::from_str(&pv.val) {
                    Ok(b) => Ok(TypedefClause::IsClassLevel(b)),
                    Err(e) => Err(Error::InvalidBoolean(e, pv.val.to_string())),
                }
            },
            obo_in_owl::IS_METADATA_TAG => {
                match bool::from_str(&pv.val) {
                    Ok(b) => Ok(TypedefClause::IsMetadataTag(b)),
                    Err(e) => Err(Error::InvalidBoolean(e, pv.val.to_string())),
                }
            }
        )
    }
}

impl FromGraph<BasicPropertyValue>  for InstanceClause {
    fn from_graph(pv: BasicPropertyValue) -> Result<Self> {
        let s = pv.pred.as_str();
        impl_basic_pv_common!(pv, InstanceClause, s)
    }
}
