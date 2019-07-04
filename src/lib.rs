extern crate serde;

#[cfg(feature = "obo")]
mod obo;

use serde::Deserializer;
use serde::Deserialize;
use serde::Serialize;

/// Deserialize a possibly missing vector into an empty one.
fn optional_vector<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    match Option::deserialize(deserializer) {
        Ok(Some(v)) => Ok(v),
        Ok(None) => Ok(Vec::new()),
        Err(e) => Err(e),
    }
}

fn nullable_vector<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Vec::<Option<T>>::deserialize(deserializer).map(|v|
        v.into_iter().flatten().collect()
    )
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GraphDocument {
    #[serde(default, deserialize_with = "optional_vector")]
    pub graphs: Vec<Graph>,
    meta: Option<Box<Meta>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Graph {
    #[serde(default, deserialize_with = "optional_vector")]
    nodes: Vec<Node>,
    #[serde(default, deserialize_with = "optional_vector")]
    edges: Vec<Edge>,
    id: String,
    #[serde(rename = "lbl")]
    label: Option<String>,
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
    meta: Option<Box<Meta>>,
    #[serde(rename = "type")]
    ty: Option<NodeType>, // FIXME: Use `CLASS` as default instead?
    label: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Meta {
    definition: Option<Box<DefinitionPropertyValue>>,
    #[serde(default, deserialize_with = "optional_vector")]
    comments: Vec<String>,
    #[serde(default, deserialize_with = "optional_vector")]
    subsets: Vec<String>,
    #[serde(default, deserialize_with = "optional_vector")]
    xrefs: Vec<XrefPropertyValue>,
    #[serde(default, deserialize_with = "optional_vector")]
    synonyms: Vec<SynonymPropertyValue>,
    #[serde(rename = "basicPropertyValues", default, deserialize_with = "optional_vector")]
    basic_property_values: Vec<BasicPropertyValue>,
    version: Option<String>,
    #[serde(default)]
    deprecated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DefinitionPropertyValue {
    pred: Option<String>,
    val: String,
    xrefs: Vec<String>,
    meta: Option<Box<Meta>>,
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
    meta: Option<Box<Meta>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EquivalentNodesSet {
    meta: Option<Box<Meta>>,
    #[serde(rename = "representativeNodeId")]
    representative_node_id: Option<String>,
    #[serde(rename = "nodeIds", default, deserialize_with = "optional_vector")]
    node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LogicalDefinitionAxiom {
    meta: Option<Box<Meta>>,
    #[serde(rename = "definedClassId")]
    defined_class_id: String,
    #[serde(rename = "genusIds")]
    genus_ids: Vec<String>,
    #[serde(deserialize_with = "nullable_vector")]
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
    meta: Option<Box<Meta>>,
    #[serde(rename = "predicateId")]
    predicate_id: String,
    #[serde(rename = "domainClassIds", default, deserialize_with = "optional_vector")]
    domain_class_ids: Vec<String>,
    #[serde(rename = "rangeClassIds", default, deserialize_with = "optional_vector")]
    range_class_ids: Vec<String>,
    #[serde(rename = "allValuesFromEdges", default, deserialize_with = "optional_vector")]
    all_values_from_edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PropertyChainAxiom {
    meta: Option<Box<Meta>>,
    #[serde(rename = "predicateId")]
    predicate_id: String,
    #[serde(rename = "chainPredicateIds", default, deserialize_with = "optional_vector")]
    chain_predicate_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct XrefPropertyValue {
    pred: Option<String>,
    val: String,
    #[serde(default, deserialize_with = "optional_vector")]
    xrefs: Vec<String>,
    meta: Option<Box<Meta>>,
    #[serde(rename = "lbl")]
    label: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SynonymPropertyValue {
    pred: String,
    val: String,
    #[serde(default, deserialize_with = "optional_vector")]
    xrefs: Vec<String>,
    meta: Option<Box<Meta>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct BasicPropertyValue {
    pred: String,
    val: String,
    #[serde(default, deserialize_with = "optional_vector")]
    xrefs: Vec<String>,
    meta: Option<Box<Meta>>,
}
