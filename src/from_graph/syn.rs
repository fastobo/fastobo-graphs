use std::str::FromStr;

use fastobo::ast::Ident;
use fastobo::ast::QuotedString;
use fastobo::ast::Synonym;
use fastobo::ast::SynonymScope;
use fastobo::ast::Xref;
use fastobo::ast::XrefList;

use super::FromGraph;
use crate::error::Error;
use crate::error::Result;
use crate::model::SynonymPropertyValue;

impl FromGraph<SynonymPropertyValue> for Synonym {
    fn from_graph(pv: SynonymPropertyValue) -> Result<Self> {
        let desc = QuotedString::new(pv.val);
        let scope = match pv.pred.as_str() {
            "hasBroadSynonym" => SynonymScope::Broad,
            "hasExactSynonym" => SynonymScope::Exact,
            "hasNarrowSynonym" => SynonymScope::Narrow,
            "hasRelatedSynonym" => SynonymScope::Related,
            other => return Err(Error::InvalidSynonymType(other.to_string())),
        };
        let xrefs = pv
            .xrefs
            .into_iter()
            .map(|id| Ident::from_str(&id).map(Xref::new).map_err(Error::from))
            .collect::<Result<XrefList>>()?;
        Ok(Synonym::with_xrefs(desc, scope, xrefs))
    }
}
