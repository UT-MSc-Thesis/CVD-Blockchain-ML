use cosmwasm_std::{Addr, Timestamp};
use secret_toolkit_storage::{Item, Keymap};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Record {
    pub title: String,
    pub timestamp: Timestamp,
    pub description: String,
    pub data: String,
}

pub static OWNER: Item<Addr> = Item::new(b"owner");
pub static REGISTRY: Item<Addr> = Item::new(b"registry");
pub static RECORD_STORE: Keymap<String, Record> = Keymap::new(b"record");
