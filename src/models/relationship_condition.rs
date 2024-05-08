/*
 * OpenFGA
 *
 * A high performance and flexible authorization/permission engine built for developers and inspired by Google Zanzibar.
 *
 * The version of the OpenAPI document: 0.1
 * Contact: community@openfga.dev
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipCondition {
    /// A reference (by name) of the relationship condition defined in the authorization model.
    #[serde(rename = "name")]
    pub name: String,
    /// Additional context/data to persist along with the condition. The keys must match the parameters defined by the condition, and the value types must match the parameter type definitions.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

impl RelationshipCondition {
    pub fn new(name: String) -> RelationshipCondition {
        RelationshipCondition {
            name,
            context: None,
        }
    }
}
