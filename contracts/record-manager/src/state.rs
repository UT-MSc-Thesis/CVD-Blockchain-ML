use cosmwasm_std::{Addr, Timestamp};
use secret_toolkit_storage::Item;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub title: String,
    pub timestamp: Timestamp,
    pub description: String,
    pub sex: String,
    pub cholestrol: u32,
    pub trestbps: u32,
}

pub static OWNER: Item<Addr> = Item::new(b"owner");
// pub static RECORD_STORE: Keymap<String, Record> = Keymap::new(b"record");
