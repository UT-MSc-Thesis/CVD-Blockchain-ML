use cosmwasm_std::Addr;
use secret_toolkit::utils::InitCallback;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub offspring_id: u64,
    pub offspring_hash: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ExecuteMsg {
    Register { id: String, address: Addr },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    Info { id: String },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InfoResp {
    pub address: Addr,
    pub contract_address: Addr,
}

#[derive(Deserialize)]
pub struct OffspringResp {
    pub offspring_address: Addr,
    pub owner_id: String,
    pub owner_address: Addr,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct OffspringInstantiateMsg {
    pub owner: Addr,
    pub owner_id: String,
}

impl InitCallback for OffspringInstantiateMsg {
    const BLOCK_SIZE: usize = 256;
}
