use cosmwasm_std::Addr;
use schemars::JsonSchema;
use secret_toolkit::permit::Permit;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub owner_id: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddRecord {
        id: String,
        title: String,
        description: String,
        data: String,
        permit: Permit<RecordPermissions>,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ViewById {
        permit: Permit<RecordPermissions>,
        record_id: String,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct CallbackInfo {
    pub offspring_address: Addr,
    pub owner_id: String,
    pub owner_address: Addr,
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecordPermissions {
    ViewById { record_id: String },
    Add,
}
