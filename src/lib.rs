extern crate serde;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GraphDocument {
    graphs: Vec<Graph>,
    meta: Box<Meta>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    id: String,
    #[serde(rename = "lbl")]
    label: String,
    meta: Box<Meta>,
    #[serde(rename = "equivalentNodesSets")]
    equivalent_nodes_sets: Vec<EquivalentNodesSet>,
    #[serde(rename = "logicalDefinitionAxioms")]
    logical_definition_axioms: Vec<LogicalDefinitionAxiom>,
    #[serde(rename = "domainRangeAxioms")]
    domain_range_axioms: Vec<DomainRangeAxiom>,
    #[serde(rename = "propertyChainAxioms")]
    property_chain_axioms: Vec<PropertyChainAxiom>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Node {
    id: String,
    meta: Box<Meta>,
    #[serde(rename = "type")]
    ty: NodeType,
    label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Meta {
    definition: DefinitionPropertyValue,
    comments: Vec<String>,
    subsets: Vec<String>,
    xrefs: Vec<XrefPropertyValue>,
    synonyms: Vec<SynonymPropertyValue>,
    #[serde(rename = "basicPropertyValues")]
    basic_property_values: Vec<BasicPropertyValue>,
    version: String,
    deprecated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DefinitionPropertyValue {
    pred: String,
    val: String,
    xrefs: String,
    meta: Box<Meta>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeType {
    Class,
    Individual,
    Property,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    sub: String,
    pred: String,
    obj: String,
    meta: Box<Meta>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EquivalentNodesSet {
    meta: Box<Meta>,
    #[serde(rename = "representativeNodeId")]
    representative_node_id: String,
    #[serde(rename = "nodeIds")]
    node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LogicalDefinitionAxiom {
    meta: Box<Meta>,
    #[serde(rename = "definedClassId")]
    defined_class_id: String,
    #[serde(rename = "genusIds")]
    genus_ids: Vec<String>,
    restrictions: Vec<ExistentialRestrictionExpression>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ExistentialRestrictionExpression {
    #[serde(rename = "propertyId")]
    property_id: String,
    #[serde(rename = "fillerId")]
    filler_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DomainRangeAxiom {
    meta: Box<Meta>,
    #[serde(rename = "predicateId")]
    predicate_id: String,
    #[serde(rename = "domainClassIds")]
    domain_class_ids: Vec<String>,
    #[serde(rename = "rangeClassIds")]
    range_class_ids: Vec<String>,
    #[serde(rename = "allValuesFromEdges")]
    all_values_from_edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PropertyChainAxiom {
    meta: Box<Meta>,
    #[serde(rename = "predicateId")]
    predicate_id: String,
    #[serde(rename = "chainPredicateIds")]
    chain_predicate_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct XrefPropertyValue {
    pred: String,
    val: String,
    xrefs: Vec<String>,
    meta: Box<Meta>,
    #[serde(rename = "lbl")]
    label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SynonymPropertyValue {
    pred: String,
    val: String,
    xrefs: Vec<String>,
    meta: Box<Meta>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct BasicPropertyValue {
    pred: String,
    val: String,
    xrefs: Vec<String>,
    meta: Box<Meta>,
}
