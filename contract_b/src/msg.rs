use cosmwasm_schema::{cw_serde, schemars::JsonSchema};
use serde::{Deserialize, Serialize};
#[cw_serde]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ReceiveCall { message: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetMessages {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MessagesResponse {
    pub messages: Vec<String>,
}
