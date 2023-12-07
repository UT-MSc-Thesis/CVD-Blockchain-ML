use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("{sender} is not authorized to perform the requested action")]
    Unauthorized { sender: Addr },
    #[error("Reply id {id} was not expected")]
    UnexpectedReplyId { id: u64 },
    #[error("Failed to instantiate a Record Manager contract")]
    OffspringInstantiationError {},
    #[error("Error: {val:?}")]
    CustomError { val: String },
}
