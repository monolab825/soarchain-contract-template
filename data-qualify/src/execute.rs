
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Response;


use crate::error::ContractError;


// Struct for input data for a single connection
#[cw_serde]
pub struct ConnectionInput {
    pub operation: FullOperation,
    pub source_chain: String,
}

// Enum to represent the operation to be performed (including enable/disable)
#[cw_serde]
pub enum FullOperation {
    Set,
}

// Set, change, or remove a source chain, destination chain, and channel connection
pub fn connection_operations(
    operations: Vec<ConnectionInput>,
) -> Result<Response, ContractError> {
    let response = Response::new();
    for operation in operations {
        let source_chain = operation.source_chain.to_lowercase();
        let destination_chain = "soarchain".to_string();

        match operation.operation {
            FullOperation::Set => {
                response.clone().add_attribute(
                    "set_connection",
                    format!("{source_chain}-{destination_chain}"),
                );
            }
        }
    }
    Ok(response)
}