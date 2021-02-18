use fastobo::ast::PropertyValue;

use crate::error::Result;
use crate::model::BasicPropertyValue;

use super::Context;
use super::IntoGraphCtx;

impl IntoGraphCtx<BasicPropertyValue> for PropertyValue {
    fn into_graph_ctx(self, ctx: &mut Context) -> Result<BasicPropertyValue> {
        match self {
            PropertyValue::Resource(pv) => Ok(BasicPropertyValue::new(
                ctx.expand(pv.property()),
                ctx.expand(pv.target()),
            )),
            PropertyValue::Literal(pv) => Ok(BasicPropertyValue::new(
                ctx.expand(pv.property()),
                pv.literal().as_str().to_string(),
            )),
        }
    }
}
