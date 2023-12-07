use cosmwasm_std::Addr;
use secret_toolkit_storage::Keymap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub address: Addr,
}

pub static PERSON_STORE: Keymap<String, Person> = Keymap::new(b"person");
