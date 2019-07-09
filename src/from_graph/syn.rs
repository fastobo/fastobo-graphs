use std::str::FromStr;

use fastobo::ast::Synonym;
use fastobo::ast::SynonymScope;
use fastobo::ast::Ident;
use fastobo::ast::Xref;
use fastobo::ast::XrefList;
use fastobo::ast::QuotedString;

use crate::model::SynonymPropertyValue;
use super::FromGraph;

impl FromGraph<SynonymPropertyValue> for Synonym {
    fn from_graph(pv: SynonymPropertyValue) -> Self {
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
