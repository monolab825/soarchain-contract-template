#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, entry_point, StdResult, to_json_binary, Binary,
};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::types::RoadUsageCharge;
use crate::msg::{ RoadUsageChargeResponse, InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:road-usage-charge-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const TOLL_PRICE: u64 = 100;
const CONVERSION_FACTORE: u64 = 160934;

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
        charging: msg.charging,
        calculated_charge: String::new()
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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    // Implement your logic to handle different execute messages here
    // This is just a placeholder, replace it with your actual implementation

    match msg {

        // Your custom logic goes here
        ExecuteMsg::RoadUsageCharge { charging } => Ok(execute::road_usage(deps, env, info, charging)?),
    }
}

pub mod execute {
    use super::*;

    // Implement the proper helper functions match with your needs
    // This is just a placeholder, replace it with your actual implementation

    pub fn road_usage(deps: DepsMut, env: Env, info: MessageInfo, charging: RoadUsageCharge) -> Result<Response, ContractError> {

        // Implement the smart contract with replacing your logic
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }

            if charging.source_data.data_info.data_details.vehicle_info.odometer >= charging.target_data.data_info.data_details.vehicle_info.odometer {
                return Err(ContractError::NoValidOdometer {});
            }

            let price: u64 = calculate_distance(charging.source_data.data_info.data_details.vehicle_info.odometer, charging.target_data.data_info.data_details.vehicle_info.odometer) * TOLL_PRICE;

            if meets_condition(&env, &info, &charging) {

                state.charging = charging;
                state.calculated_charge = make_calculation_price(price.to_string()) ;
                Ok(state)
                
            } else {
                Err(ContractError::FaildTollCalculation {})
            }
        })?;

        Ok(Response::new().add_attribute("action", "road_usage"))
    }

    pub fn meets_condition(_env: &Env, _info: &MessageInfo, charging: &RoadUsageCharge) -> bool {

        // Implement your logic for checking conditions
        // This can involve checking sender, time, or any other parameters
        // For simplicity, let's assume the condition is always met

        if charging.source_data.data_info.data_details.vehicle_info.odometer <= 0  || 
        charging.target_data.data_info.data_details.vehicle_info.odometer <= 0
        {
            return false
        }

        return true;
    }

    fn calculate_distance(source_odometer: u64, target_odometer: u64) -> u64 {

        let source_kilometers = source_odometer * CONVERSION_FACTORE ;
        let target_kilometers = target_odometer * CONVERSION_FACTORE ;

        return target_kilometers - source_kilometers;
    }

    fn make_calculation_price(price: String) -> String{

        // Get the length of the original string
        let len = price.len();
        let mut result: String = String::new();
        if len >= 5 {
            // Use string slicing to get the last 5 characters
            let last_five_chars = &price[len - 5..];
            let remaining_chars = &price[..len - 5];
            result = remaining_chars.to_owned() + "." + last_five_chars;
        } 
        return result;
    }

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {

    // Implement your query logic here
    // This is just a placeholder, replace it with your actual implementation

    match msg {

        // Your custom query logic goes here
        QueryMsg::GetRoadUsageCharge {} => to_json_binary(&query::get_road_usage_charge(deps,)?),
    }
}
pub mod query {

    use super::*;

    pub fn get_road_usage_charge(deps: Deps) -> StdResult<RoadUsageChargeResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(RoadUsageChargeResponse { charge: state.calculated_charge })
    }

 }



