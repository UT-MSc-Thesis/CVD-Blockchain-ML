use cosmwasm_std::Addr;
use secret_toolkit::storage::{Item, Keymap};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Person {
    pub address: Addr,
    pub contract_address: Addr,
    pub pubkey: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct OffspringInfo {
    pub code_id: u64,
    pub code_hash: String,
}

pub static OWNER: Item<Addr> = Item::new(b"owner");
pub static OFFSPRING: Item<OffspringInfo> = Item::new(b"offspring");
pub static PERSON_STORE: Keymap<String, Person> = Keymap::new(b"person");
