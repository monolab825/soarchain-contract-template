use cosmwasm_std::StdError;
use thiserror::Error;

#[derive( Error, Debug, PartialEq)]
pub enum ContractError {
    // Add more errors to check your logic

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Data not found")]
    NoFound {},

    #[error("Anonymization failed!")]
    AnonymizedFailed {},

}
