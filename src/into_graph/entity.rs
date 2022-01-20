use fastobo::ast::EntityFrame;
use fastobo::ast::InstanceFrame;
use fastobo::ast::TermFrame;
use fastobo::ast::TypedefFrame;

use super::Context;
use super::IntoGraphCtx;
use crate::constants::property::obo_in_owl;
use crate::error::Result;
use crate::model::BasicPropertyValue;
use crate::model::DefinitionPropertyValue;
use crate::model::DomainRangeAxiom;
use crate::model::Edge;
use crate::model::Graph;
use crate::model::Meta;
use crate::model::Node;
use crate::model::NodeType;
use crate::model::XrefPropertyValue;

// ---------------------------------------------------------------------------

macro_rules! impl_frame_common {
    (
        $ctx:ident,
        $clause:ident,
        $node:ident,
        $edges:ident,
        $meta:ident,
        $current:ident
        $(, $l:pat => $r:expr )*
    ) => ({
        match $clause {
            IsAnonymous(val) => (),
            Name(name) => {
                $node.label = Some(name.into_string());
            }
            Namespace(ns) => {
                $meta.basic_property_values.push(
                    BasicPropertyValue::new(
                        obo_in_owl::HAS_OBO_NAMESPACE.to_string(),
                        ns.to_string(),
                    )
                );
            }
            AltId(alt_id) => {
                $meta.basic_property_values.push(
                    BasicPropertyValue::new(
                        obo_in_owl::HAS_ALTERNATIVE_ID.to_string(),
                        alt_id.to_string(),
                    )
                );
            }
            Def(def) => {
                $meta.definition = Some(Box::new(
                    DefinitionPropertyValue {
                        pred: None,
                        val: def.text().as_str().to_string(),
                        xrefs: def.xrefs().iter().map(|x| $ctx.expand(x.id())).collect(),
                        meta: None
                    }
                ))
            }
            Comment(comment) => {}
            Subset(subset) => {}
            Synonym(syn) => {}
            Xref(xref) => {
                $meta.xrefs.push(
                    XrefPropertyValue {
                        pred: None,
                        val:  $ctx.expand(xref.id()),
                        xrefs: Vec::new(),
                        meta: None,
                        label: xref.description().map(|d| d.clone().into_string()),
                    }
                )
            }
            Builtin(bool) => {}
            PropertyValue(pv) => {}
            IsA(id) => {
                $edges.push(
                    Edge {
                        sub: $current.clone(),
                        pred: String::from("is_a"),
                        obj: $ctx.expand(*id),
                        meta: None,
                    }
                );
            }
            UnionOf(cid) => {}
            EquivalentTo(cid) => {}
            DisjointFrom(cid) => {}
            Relationship(rid, cid) => {
                $edges.push(
                    Edge {
                        sub: $current.clone(),
                        pred: $ctx.expand(*rid),
                        obj: $ctx.expand(*cid),
                        meta: None,
                    }
                )
            }
            CreatedBy(name) => {
                $meta.basic_property_values.push(
                    BasicPropertyValue::new(
                        obo_in_owl::CREATED_BY.to_string(),
                        name.into_string(),
                    )
                );
            }
            CreationDate(dt) => {
                $meta.basic_property_values.push(
                    BasicPropertyValue::new(
                        obo_in_owl::CREATION_DATE.to_string(),
                        dt.to_string(),
                    )
                );
            }
            IsObsolete(val) => {}
            ReplacedBy(cid) => {
                $meta.basic_property_values.push(
                    BasicPropertyValue::new(
                        obo_in_owl::REPLACED_BY.to_string(),
                        $ctx.expand(*cid),
                    )
                );
            }
            Consider(cid) => {
                $meta.basic_property_values.push(
                    BasicPropertyValue::new(
                        obo_in_owl::CONSIDER.to_string(),
                        $ctx.expand(*cid),
                    )
                );
            }
            $( $l => $r ),*
        }
    });
}

// ---------------------------------------------------------------------------

impl IntoGraphCtx<Graph> for EntityFrame {
    fn into_graph_ctx(self, ctx: &mut Context) -> Result<Graph> {
        match self {
            EntityFrame::Term(t) => t.into_graph_ctx(ctx),
            EntityFrame::Typedef(t) => t.into_graph_ctx(ctx),
            EntityFrame::Instance(t) => t.into_graph_ctx(ctx),
        }
    }
}

// ---------------------------------------------------------------------------

impl IntoGraphCtx<Graph> for TermFrame {
    fn into_graph_ctx(self, ctx: &mut Context) -> Result<Graph> {
        use fastobo::ast::TermClause::*;

        //
        let mut edges = Vec::new();
        let mut meta = Meta::default();
        let mut node = Node {
            id: ctx.expand(self.id().as_inner()),
            meta: None,
            ty: Some(NodeType::Class),
            label: None,
        };

        //
        let current_id = ctx.expand(self.id().as_inner());
        for line in self.into_iter() {
            let clause = line.into_inner();
            impl_frame_common!(ctx, clause, node, edges, meta, current_id,
                IntersectionOf(optrid, cid) => {}
            );
        }

        //
        node.meta = Some(Box::new(meta));
        Ok(Graph {
            id: node.id.clone(),
            nodes: vec![node],
            edges,
            label: None,
            meta: Box::new(Meta::default()),
            equivalent_nodes_sets: Vec::new(),
            logical_definition_axioms: Vec::new(),
            domain_range_axioms: Vec::new(),
            property_chain_axioms: Vec::new(),
        })
    }
}

// ---------------------------------------------------------------------------

impl IntoGraphCtx<Graph> for TypedefFrame {
    fn into_graph_ctx(self, ctx: &mut Context) -> Result<Graph> {
        use fastobo::ast::TypedefClause::*;

        let mut edges = Vec::new();
        let mut meta = Meta::default();
        let mut node = Node {
            id: ctx.expand(self.id().as_inner()),
            meta: None,
            ty: Some(NodeType::Property),
            label: None,
        };
        let mut dra = Vec::with_capacity(1);

        let current_id = ctx.expand(self.id().as_inner());
        for line in self.into_iter() {
            let clause = line.into_inner();
            impl_frame_common!(ctx, clause, node, edges, meta, current_id,
                Domain(id) => {
                    if dra.is_empty() {
                        dra.push(DomainRangeAxiom {
                            meta: None,
                            predicate_id: current_id.clone(),
                            domain_class_ids: Vec::new(),
                            range_class_ids: Vec::new(),
                            all_values_from_edges: Vec::new(),
                        });
                    }
                    dra[0].domain_class_ids.push(ctx.expand(*id));
                },
                Range(id) => {
                    if dra.is_empty() {
                        dra.push(DomainRangeAxiom {
                            meta: None,
                            predicate_id: current_id.clone(),
                            domain_class_ids: Vec::new(),
                            range_class_ids: Vec::new(),
                            all_values_from_edges: Vec::new(),
                        });
                    }
                    dra[0].range_class_ids.push(ctx.expand(*id));
                },
                HoldsOverChain(r1, r2) => {},
                IsAntiSymmetric(b) => {},
                IsCyclic(b) => {
                    meta.basic_property_values.push(
                        BasicPropertyValue::new(
                            obo_in_owl::IS_CYCLIC.to_string(),
                            b.to_string(),
                        )
                    );
                },
                IsReflexive(b) => {},
                IsSymmetric(b) => {},
                IsAsymmetric(b) => {},
                IsTransitive(b) => {},
                IsFunctional(b) => {},
                IsInverseFunctional(b) => {},
                IntersectionOf(rid) => {},
                InverseOf(r) => {},
                TransitiveOver(r) => {},
                EquivalentToChain(r1, r2) => {},
                DisjointOver(r) => {
                    meta.basic_property_values.push(
                        BasicPropertyValue::new(
                            obo_in_owl::DISJOINT_OVER.to_string(),
                            ctx.expand(*r),
                        )
                    );
                },
                ExpandAssertionTo(desc, xrefs) => {},
                ExpandExpressionTo(desc, xrefs) => {},
                IsMetadataTag(b) => {
                    meta.basic_property_values.push(
                        BasicPropertyValue::new(
                            obo_in_owl::IS_METADATA_TAG.to_string(),
                            b.to_string(),
                        )
                    );
                },
                IsClassLevel(b) => {
                    meta.basic_property_values.push(
                        BasicPropertyValue::new(
                            obo_in_owl::IS_CLASS_LEVEL.to_string(),
                            b.to_string(),
                        )
                    );
                }
            );
        }

        Ok(Graph {
            edges,
            id: node.id.clone(),
            nodes: vec![node],
            label: None,
            meta: Box::new(Meta::default()),
            domain_range_axioms: dra,
            equivalent_nodes_sets: Vec::new(),
            logical_definition_axioms: Vec::new(),
            property_chain_axioms: Vec::new(),
        })
    }
}

// ---------------------------------------------------------------------------

impl IntoGraphCtx<Graph> for InstanceFrame {
    fn into_graph_ctx(self, ctx: &mut Context) -> Result<Graph> {
        // ... TODO ... //
        Ok(Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            id: ctx.expand(self.id().as_ref()),
            label: None,
            meta: Box::new(Meta::default()),
            equivalent_nodes_sets: Vec::new(),
            logical_definition_axioms: Vec::new(),
            domain_range_axioms: Vec::new(),
            property_chain_axioms: Vec::new(),
        })
    }
}
