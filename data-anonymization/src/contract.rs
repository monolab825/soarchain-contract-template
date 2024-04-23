#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, entry_point, Binary, StdResult, to_json_binary,
};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{DataAnonymizationResultResponse, AnonymizedDataResponse, InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{State, STATE};
use core::str;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:data-anonymization-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    // You can initialize any state or perform setup logic here

    let state = State {
        owner: info.sender.to_string(),
        json_data: msg.data
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender.to_string())
    )
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

        if msg.data == "" {
            return Err(ContractError::NoFound {});
        }

        // Implement the smart contract with replacing your logic
        // This is just a placeholder, replace it with your actual implementation

        STATE.update(deps.storage, | mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }

            // Decode the hex string into a byte vector
            let byte_vector = hex::decode(&msg.data).expect("Failed to decode hex string");

            // Convert the byte vector back to a hex string
            let converted_hex_string = hex::encode(&byte_vector);

            if converted_hex_string != msg.data {
                return Err(ContractError::AnonymizedFailed {});
            }

            // Convert the byte vector to a original string
            let original_string = String::from_utf8_lossy(&byte_vector);

            state.json_data = original_string.to_string();
            Ok(state)

        })?;

        Ok(Response::new().add_attribute("action", "data_anonymization"))
}

/** 
 * 
 *  Implement your algorithm to handle encreapt/decreapt here
 *  This is just a placeholder, replace it with your actual implementation

pub mod execute {
    use super::*;
    
    pub fn execute_data_anonymization(deps: DepsMut, info: MessageInfo, json_string: String) -> Result<Response, ContractError> {
        Implement the smart contract with replacing your logic
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {

            //Implement your algorithm to handle encreapt/decreapt here
            //This is just a placeholder, replace it with your actual implementation

            let serialized = serde_json::to_string(&data).expect("Serialization failed");
            let payload = serialized.as_bytes();
        
            // Sample password used for encryption and decryption
            let password = b"your password";
        
            // Encrypt the payload using the password
            let encrypted = simplestcrypt::encrypt_and_serialize(&password[..], &payload).unwrap();
        
            // Decrypt the encrypted data using the password
            let decreapted = simplestcrypt::deserialize_and_decrypt(&password[..], &encrypted).unwrap();
        
            if payload != decreapted {
                return Err(ContractError::AnonymizedFailed {});
            }

            let decreapted_data: Data = serde_json::from_slice(&payload).unwrap();

            state.json_data = decreapted_data;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "data_anonymization"))
    }
}

*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {

    // Implement your query logic here
    // This is just a placeholder, replace it with your actual implementation

    match msg {

        // Your custom query logic goes here
        QueryMsg::GetAnonymizedResult {} => to_json_binary(&query::get_anonymization_result(deps,)?),
        QueryMsg::GetAnonymizedData {} => to_json_binary(&query::get_anonymized_data(deps,)?),
    }
}

pub mod query {

    use super::*;

    pub fn get_anonymization_result(deps: Deps) -> StdResult<DataAnonymizationResultResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(DataAnonymizationResultResponse { anonymized: anonymization_result(state.json_data).anonymized })
    }

    fn anonymization_result(data: String) -> DataAnonymizationResultResponse {

        let mut result: bool = true;

        // Replace it with your actual implementation

        if data == "" {
            result = false;
        }
    
        DataAnonymizationResultResponse {
            anonymized: result,
        }
    }

    pub fn get_anonymized_data(deps: Deps) -> StdResult<AnonymizedDataResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(AnonymizedDataResponse { data: state.json_data})
    }
 
 }



