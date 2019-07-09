use std::str::FromStr;

use fastobo::ast::Xref;
use fastobo::ast::Ident;

use crate::model::XrefPropertyValue;
use super::FromGraph;

impl FromGraph<XrefPropertyValue> for Xref {
    fn from_graph(pv: XrefPropertyValue) -> Self {
        // FIXME: what to do with label ? what to do with meta ?
        let id = Ident::from_str(&pv.val).expect("invalid xref ident");
        Xref::new(id)
    }
}
