mod doc;
mod entity;
mod header;
mod pv;

pub struct Context;

pub trait IntoGraphCtx {
    type OboGraph;
    fn into_graph_ctx(self, ctx: &mut Context) -> Self::OboGraph;
}
