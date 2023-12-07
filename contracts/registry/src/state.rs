use cosmwasm_std::Addr;
use secret_toolkit::storage::{Item, Keymap};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub address: Addr,
    pub contract_address: Addr,
}

pub static OWNER: Item<Addr> = Item::new(b"owner");
pub static PERSON_STORE: Keymap<String, Person> = Keymap::new(b"person");
