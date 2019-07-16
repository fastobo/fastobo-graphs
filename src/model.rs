use serde::Deserializer;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::serde::optional_vector;
use crate::utils::serde::nullable_vector;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GraphDocument {
    #[serde(default, deserialize_with = "optional_vector")]
    pub graphs: Vec<Graph>,
    pub meta: Option<Box<Meta>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Graph {
    #[serde(default, deserialize_with = "optional_vector")]
    pub nodes: Vec<Node>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub edges: Vec<Edge>,
    pub id: String,
    #[serde(rename = "lbl")]
    pub label: Option<String>,
    pub meta: Box<Meta>,
    #[serde(rename = "equivalentNodesSets")]
    pub equivalent_nodes_sets: Vec<EquivalentNodesSet>,
    #[serde(rename = "logicalDefinitionAxioms")]
    pub logical_definition_axioms: Vec<LogicalDefinitionAxiom>,
    #[serde(rename = "domainRangeAxioms")]
    pub domain_range_axioms: Vec<DomainRangeAxiom>,
    #[serde(rename = "propertyChainAxioms")]
    pub property_chain_axioms: Vec<PropertyChainAxiom>,
}

impl Graph {
    pub fn extend(&mut self, other: Self) {
        self.nodes.extend(other.nodes);
        self.edges.extend(other.edges);
        self.equivalent_nodes_sets.extend(other.equivalent_nodes_sets);
        self.logical_definition_axioms.extend(other.logical_definition_axioms);
        self.domain_range_axioms.extend(other.domain_range_axioms);
        self.property_chain_axioms.extend(other.property_chain_axioms);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub meta: Option<Box<Meta>>,
    #[serde(rename = "type")]
    pub ty: Option<NodeType>, // FIXME: Use `CLASS` as default instead?
    #[serde(rename = "lbl")]
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct Meta {
    pub definition: Option<Box<DefinitionPropertyValue>>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub comments: Vec<String>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub subsets: Vec<String>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub xrefs: Vec<XrefPropertyValue>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub synonyms: Vec<SynonymPropertyValue>,
    #[serde(rename = "basicPropertyValues", default, deserialize_with = "optional_vector")]
    pub basic_property_values: Vec<BasicPropertyValue>,
    pub version: Option<String>,
    #[serde(default)]
    pub deprecated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DefinitionPropertyValue {
    pub pred: Option<String>,
    pub val: String,
    pub xrefs: Vec<String>,
    pub meta: Option<Box<Meta>>,
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
    pub sub: String,
    pub pred: String,
    pub obj: String,
    pub meta: Option<Box<Meta>>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct EquivalentNodesSet {
    pub meta: Option<Box<Meta>>,
    #[serde(rename = "representativeNodeId")]
    pub representative_node_id: Option<String>,
    #[serde(rename = "nodeIds", default, deserialize_with = "optional_vector")]
    pub node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct LogicalDefinitionAxiom {
    pub meta: Option<Box<Meta>>,
    #[serde(rename = "definedClassId")]
    pub defined_class_id: String,
    #[serde(rename = "genusIds")]
    pub genus_ids: Vec<String>,
    #[serde(deserialize_with = "nullable_vector")]
    pub restrictions: Vec<ExistentialRestrictionExpression>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct ExistentialRestrictionExpression {
    #[serde(rename = "propertyId")]
    pub property_id: String,
    #[serde(rename = "fillerId")]
    pub filler_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DomainRangeAxiom {
    pub meta: Option<Box<Meta>>,
    #[serde(rename = "predicateId")]
    pub predicate_id: String,
    #[serde(rename = "domainClassIds", default, deserialize_with = "optional_vector")]
    pub domain_class_ids: Vec<String>,
    #[serde(rename = "rangeClassIds", default, deserialize_with = "optional_vector")]
    pub range_class_ids: Vec<String>,
    #[serde(rename = "allValuesFromEdges", default, deserialize_with = "optional_vector")]
    pub all_values_from_edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PropertyChainAxiom {
    pub meta: Option<Box<Meta>>,
    #[serde(rename = "predicateId")]
    pub predicate_id: String,
    #[serde(rename = "chainPredicateIds", default, deserialize_with = "optional_vector")]
    pub chain_predicate_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct XrefPropertyValue {
    pub pred: Option<String>,
    pub val: String,
    #[serde(default, deserialize_with = "optional_vector")]
    pub xrefs: Vec<String>,
    pub meta: Option<Box<Meta>>,
    #[serde(rename = "lbl")]
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SynonymPropertyValue {
    pub pred: String,
    pub val: String,
    #[serde(default, deserialize_with = "optional_vector")]
    pub xrefs: Vec<String>,
    pub meta: Option<Box<Meta>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct BasicPropertyValue {
    pub pred: String,
    pub val: String,
    #[serde(default, deserialize_with = "optional_vector")]
    pub xrefs: Vec<String>,
    pub meta: Option<Box<Meta>>,
}

impl BasicPropertyValue {
    pub fn new(predicate: String, value: String) -> Self {
        Self {
            pred: predicate,
            val: value,
            xrefs: Vec::new(),
            meta: None,
        }
    }
}
