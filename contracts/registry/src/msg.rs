use cosmwasm_std::{to_binary, Addr, Binary, CosmosMsg, StdResult, Timestamp, WasmMsg};
use schemars::JsonSchema;
use secret_toolkit::{permit::Permit, utils::InitCallback};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub offspring_id: u64,
    pub offspring_hash: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Register {
        id: String,
        address: Addr,
        pubkey: String,
        key: String,
    },
    AddRecord {
        patient_id: String,
        record_id: String,
        record: Record,
        permit: Permit<RecordPermissions>,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Info {
        id: String,
        key: String,
    },
    WithPermit {
        id: String,
        permit: Permit<RecordPermissions>,
        query: QueryWithPermit,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct InfoResp {
    pub address: Addr,
    pub contract_address: Addr,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct OffspringInstantiateMsg {
    pub owner: Addr,
    pub owner_id: String,
    pub owner_pubkey: String,
    pub key: String,
}

impl InitCallback for OffspringInstantiateMsg {
    const BLOCK_SIZE: usize = 256;
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct OffspringResp {
    pub offspring_address: Addr,
    pub owner_id: String,
    pub owner_address: Addr,
    pub owner_pubkey: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OffspringExecuteMsg {
    AddRecord(AddRecordMsg),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AddRecordMsg {
    pub id: String,
    pub title: String,
    pub description: String,
    pub data: String,
    pub permit: Permit<RecordPermissions>,
}

impl AddRecordMsg {
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = OffspringExecuteMsg::AddRecord(self);
        to_binary(&msg)
    }

    pub fn into_cosmos_msg<T: Into<String>, C>(
        self,
        contract_addr: T,
        code_hash: String,
    ) -> StdResult<CosmosMsg<C>>
    where
        C: Clone + std::fmt::Debug + PartialEq,
    {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            code_hash: code_hash,
            msg,
            funds: vec![],
        };
        Ok(execute.into())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OffspringQueryMsg {
    ViewById {
        permit: Permit<RecordPermissions>,
        record_id: String,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Record {
    pub title: String,
    pub timestamp: Option<Timestamp>,
    pub description: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecordPermissions {
    ViewById { record_id: String },
    Add,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryWithPermit {
    ViewById { record_id: String },
    Add,
}
