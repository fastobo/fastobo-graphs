use std::mem::replace;

use fastobo::ast::HeaderFrame;
use fastobo::ast::OboDoc;

use super::Context;
use super::IntoGraph;
use super::IntoGraphCtx;
use crate::error::Result;
use crate::model::Graph;
use crate::model::GraphDocument;

// FIXME: one graph per import, final = graph document ?
impl IntoGraphCtx<GraphDocument> for OboDoc {
    fn into_graph_ctx(mut self, ctx: &mut Context) -> Result<GraphDocument> {
        // Preprocess the document if it contains *treat-xrefs* macros.
        self.treat_xrefs();

        // Take ownership over the header and the entities.
        let header = replace(self.header_mut(), HeaderFrame::default());
        let entities = replace(self.entities_mut(), Vec::new());

        // Build the empty graph
        let mut graph = Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            id: ctx.ontology_iri.to_string(),
            label: None,
            meta: header.into_graph_ctx(ctx).map(Box::new)?,
            equivalent_nodes_sets: Vec::new(),
            logical_definition_axioms: Vec::new(),
            domain_range_axioms: Vec::new(),
            property_chain_axioms: Vec::new(),
        };

        // Extend the graph with all entities
        for entity in entities.into_iter() {
            // let mut entity_graph = entity.into_graph_ctx(ctx)?;
            // graph.extend(entity_graph);
            let entity_graph = entity.into_graph_ctx(ctx)?;
            graph.extend(entity_graph);
        }

        // TODO: Add imports recursively
        // for clause in header.iter() {}
        Ok(GraphDocument::from(graph))
    }
}

impl IntoGraph for OboDoc {
    #[inline]
    fn into_graph(self) -> Result<GraphDocument> {
        let mut ctx = Context::from(&self);
        self.into_graph_ctx(&mut ctx)
    }
}
