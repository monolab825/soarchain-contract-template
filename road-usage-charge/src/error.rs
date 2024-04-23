use cosmwasm_std::StdError;
use thiserror::Error;

#[derive( Error, Debug, PartialEq)]
pub enum ContractError {
    // Add more errors to check your logic

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Data not founs")]
    NoFunds {},

    #[error("Parameters not valid. target_data.odometer is less than source_data.odometer.")]
    NoValidOdometer {},

    #[error("Failed to calculate the toll price")]
    FaildTollCalculation {},
}
