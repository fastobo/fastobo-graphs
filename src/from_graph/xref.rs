use std::str::FromStr;

use fastobo::ast::Xref;
use fastobo::ast::Ident;

use crate::model::XrefPropertyValue;
use crate::error::Error;
use crate::error::Result;
use super::FromGraph;

impl FromGraph<XrefPropertyValue> for Xref {
    fn from_graph(pv: XrefPropertyValue) -> Result<Self> {
        // FIXME: what to do with label ? what to do with meta ?
        match Ident::from_str(&pv.val) {
            Ok(id) => Ok(Xref::new(id)),
            Err(e) => Err(Error::OboSyntaxError(e)),
        }
    }
}
