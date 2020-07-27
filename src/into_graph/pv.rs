use fastobo::ast::PropertyValue;

use crate::error::Result;
use crate::model::BasicPropertyValue;

use super::Context;
use super::IntoGraphCtx;

impl IntoGraphCtx<BasicPropertyValue> for PropertyValue {
    fn into_graph_ctx(self, ctx: &mut Context) -> Result<BasicPropertyValue> {
        match self {
            PropertyValue::Resource(rel, id) => {
                Ok(BasicPropertyValue::new(ctx.expand(rel), ctx.expand(id)))
            }
            PropertyValue::Literal(rel, value, ty) => Ok(BasicPropertyValue::new(
                ctx.expand(rel),
                value.into_string(),
            )),
        }
    }
}
