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

impl FromGraph<Meta> for HeaderFrame {
    fn from_graph(meta: Meta) -> Result<Self> {
        let mut frame = Self::new();
        // ... TODO ... //
        Ok(frame)
    }
}

impl FromGraph<BasicPropertyValue> for HeaderClause {
    fn from_graph(pv: BasicPropertyValue) -> Result<Self> {
        match pv.pred.as_str() {
            obo_in_owl::HAS_OBO_FORMAT_VERSION => {
                Ok(HeaderClause::FormatVersion(UnquotedString::new(pv.val)))
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
                Ok(HeaderClause::PropertyValue(pv))
            },
        }
    }
}
