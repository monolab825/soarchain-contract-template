use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::types::RoadUsageCharge;

#[cw_serde]
pub struct InstantiateMsg {
    // You can initialize any state or perform setup logic here
    pub charging: RoadUsageCharge,
}

// Define your custom struct for messages
#[cw_serde]
pub enum ExecuteMsg {
    RoadUsageCharge { 
        /* define parameters */  
        charging: RoadUsageCharge 
    }, 
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

    #[returns(RoadUsageChargeResponse)]
    GetRoadUsageCharge { 
        /* define parameters */
    },
}


#[cw_serde]
pub struct RoadUsageChargeResponse {

    /* define parameters */

    pub charge: String,
}
