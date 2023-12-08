use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub owner_id: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {}

#[derive(Serialize)]
pub struct CallbackInfo {
    pub offspring_address: Addr,
    pub owner_id: String,
    pub owner_address: Addr,
    pub key: String,
}
