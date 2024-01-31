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
pub struct TupleKeyWithoutCondition {
    #[serde(rename = "user")]
    pub user: String,
    #[serde(rename = "relation")]
    pub relation: String,
    #[serde(rename = "object")]
    pub object: String,
}

impl TupleKeyWithoutCondition {
    pub fn new(user: String, relation: String, object: String) -> TupleKeyWithoutCondition {
        TupleKeyWithoutCondition {
            user,
            relation,
            object,
        }
    }
}

