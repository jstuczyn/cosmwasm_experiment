use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SomeData {
    pub id: u64,
    pub field1: String,
    pub field2: String,
    pub field3: HumanAddr,
    pub field4: String,
    pub field5: String,
    pub key_field: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GrabData { start_after: Option<String> },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PagedResponse {
    pub data: Vec<SomeData>,
    pub per_page: usize,
    pub start_next_after: Option<String>,
}
