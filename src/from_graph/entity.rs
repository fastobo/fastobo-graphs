use std::str::FromStr;
use std::string::ToString;

use fastobo::ast::ClassIdent;
use fastobo::ast::CreationDate;
use fastobo::ast::Definition;
use fastobo::ast::EntityFrame;
use fastobo::ast::Ident;
use fastobo::ast::InstanceClause;
use fastobo::ast::InstanceFrame;
use fastobo::ast::InstanceIdent;
use fastobo::ast::Line;
use fastobo::ast::LiteralPropertyValue;
use fastobo::ast::PrefixedIdent;
use fastobo::ast::PropertyValue;
use fastobo::ast::QuotedString;
use fastobo::ast::RelationIdent;
use fastobo::ast::ResourcePropertyValue;
use fastobo::ast::SubsetIdent;
use fastobo::ast::Synonym;
use fastobo::ast::TermClause;
use fastobo::ast::TermFrame;
use fastobo::ast::TypedefClause;
use fastobo::ast::TypedefFrame;
use fastobo::ast::UnquotedString;
use fastobo::ast::Xref;
use fastobo::ast::XrefList;

use crate::constants::property::dc;
use crate::constants::property::iao;
use crate::constants::property::obo_in_owl;
use crate::constants::property::rdfs;
use crate::error::Error;
use crate::error::Result;

use crate::model::BasicPropertyValue;
use crate::model::DefinitionPropertyValue;
use crate::model::Meta;
use crate::model::Node;
use crate::model::NodeType;

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
                let name = Clause::Name(Box::new(UnquotedString::new(label)));
                frame.push(Line::from(name));
            }
            if let Some(meta) = $node.meta {
                let clauses: Vec<Clause> = FromGraph::from_graph(*meta)?;
                frame.extend(clauses.into_iter().map(Line::from));
            }
            Ok(Some(EntityFrame::Variant(Box::new(frame))))
        }
    }};
}

impl FromGraph<Node> for Option<EntityFrame> {
    fn from_graph(node: Node) -> Result<Self> {
        let id = Ident::from_str(&node.id)?;
        match node.ty {
            None => Ok(None),
            Some(NodeType::Class) => impl_frame_inner!(node, id, ClassIdent, Term),
            Some(NodeType::Individual) => impl_frame_inner!(node, id, InstanceIdent, Instance),
            Some(NodeType::Property) => {
                // replace ID with `oboInOwl:shorthand` if possible.
                match impl_frame_inner!(node, id, RelationIdent, Typedef) {
                    Ok(Some(EntityFrame::Typedef(mut frame))) => {
                        if let Some((idx, _)) = frame.iter().enumerate().find(|(_, c)| {
                            if let TypedefClause::PropertyValue(pv) = c.as_inner() {
                                if let PropertyValue::Literal(lpv) = pv.as_ref() {
                                    match lpv.property().as_ref() {
                                        Ident::Url(url) => url.as_str() == obo_in_owl::SHORTHAND,
                                        _ => false,
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }) {
                            let new_id = match frame.remove(idx).into_inner() {
                                TypedefClause::PropertyValue(pv) => match *pv {
                                    PropertyValue::Resource(rpv) => {
                                        RelationIdent::from(rpv.target().clone())
                                    }
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            *frame.id_mut() = new_id.into();
                        }
                        Ok(Some(EntityFrame::Typedef(frame)))
                    }
                    other => other,
                }
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
                    clauses.push($clause::Comment(Box::new(UnquotedString::new(comment))));
                }
                for subset in meta.subsets {
                    let id = SubsetIdent::from_str(&subset)?;
                    clauses.push($clause::Subset(Box::new(id)));
                }
                for xref in meta.xrefs {
                    clauses.push($clause::Xref(Box::new(Xref::from_graph(xref)?)));
                }
                for synonym in meta.synonyms {
                    clauses.push($clause::Synonym(Box::new(Synonym::from_graph(synonym)?)));
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
    };
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
                let xrefs = pv
                    .xrefs
                    .into_iter()
                    .map(|id: String| Ident::from_str(&id).map(Xref::new).map_err(Error::from))
                    .collect::<Result<XrefList>>()?;
                Ok($clause::Def(Box::new(Definition::with_xrefs(value, xrefs))))
            }
        }
    };
}

impl_definition_pv!(TermClause);
impl_definition_pv!(TypedefClause);
impl_definition_pv!(InstanceClause);

// ---------------------------------------------------------------------------

macro_rules! impl_basic_pv_common {
    ($pv:ident, $clause:ident, $x:ident $(, $l:pat => $r:expr )* ) => {{
        match $x {
            rdfs::COMMENT => {
                Ok($clause::Comment(Box::new(UnquotedString::new($pv.val))))
            },
            obo_in_owl::HAS_ALTERNATIVE_ID => {
                let id = Ident::from_str(&$pv.val)?;
                Ok($clause::AltId(Box::new(id.into())))
            },
            obo_in_owl::HAS_OBO_NAMESPACE => {
                let id = Ident::from_str(&$pv.val)?;
                Ok($clause::Namespace(Box::new(id.into())))
            },
            obo_in_owl::CREATED_BY | dc::CREATOR => {
                Ok($clause::CreatedBy(Box::new(UnquotedString::new($pv.val))))
            }
            obo_in_owl::CREATION_DATE | dc::DATE => {
                let date = CreationDate::from_str(&$pv.val)?;
                Ok($clause::CreationDate(Box::new(date)))
            }
            iao::REPLACED_BY => {
                let id = Ident::from_str(&$pv.val)?;
                Ok($clause::ReplacedBy(Box::new(id.into())))
            }
            $( $l => $r ),*
            other => {
                let rel = RelationIdent::from_str(&other)?;
                let pv = match Ident::from_str(&$pv.val) {
                    Ok(id) => PropertyValue::from(ResourcePropertyValue::new(rel, id)),
                    Err(_) => PropertyValue::from(LiteralPropertyValue::new(
                        rel,
                        QuotedString::new($pv.val),
                        Ident::from(PrefixedIdent::new("xsd", "string"))
                    ))
                };
                Ok($clause::PropertyValue(Box::new(pv)))
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

impl FromGraph<BasicPropertyValue> for TypedefClause {
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

impl FromGraph<BasicPropertyValue> for InstanceClause {
    fn from_graph(pv: BasicPropertyValue) -> Result<Self> {
        let s = pv.pred.as_str();
        impl_basic_pv_common!(pv, InstanceClause, s)
    }
}
