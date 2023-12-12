use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("{sender} is not authorized to perform the requested action")]
    Unauthorized { sender: Addr },
    #[error("Provided permit is not valid")]
    InvalidPermit,
}
