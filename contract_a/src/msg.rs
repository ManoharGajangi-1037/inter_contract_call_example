use cosmwasm_schema::cw_serde;
use serde::{Deserialize, Serialize};
#[cw_serde]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SendMessage { message: String, target: String },
    ReceiveCall { message: String },
}
