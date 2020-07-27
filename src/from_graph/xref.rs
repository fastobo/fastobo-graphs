use std::str::FromStr;

use fastobo::ast::Ident;
use fastobo::ast::Xref;

use super::FromGraph;
use crate::error::Error;
use crate::error::Result;
use crate::model::XrefPropertyValue;

impl FromGraph<XrefPropertyValue> for Xref {
    fn from_graph(pv: XrefPropertyValue) -> Result<Self> {
        // FIXME: what to do with label ? what to do with meta ?
        Ident::from_str(&pv.val).map(Xref::new).map_err(Error::from)
    }
}
