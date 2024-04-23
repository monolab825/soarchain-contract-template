use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    // You can initialize any state or perform setup logic here
    pub data: String,
}

// Define your custom struct for messages
#[cw_serde]
pub struct ExecuteMsg {
        /* define parameters */    
    pub data: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

    #[returns(DataAnonymizationResultResponse)]
    GetAnonymizedResult { 
        /* define parameters */
    },

    #[returns(AnonymizedDataResponse)]
    GetAnonymizedData { 
        /* define parameters */
    },
    
}

// Define a custom struct for each query response
#[cw_serde]
pub struct DataAnonymizationResultResponse {
    /* define parameters */
    pub anonymized: bool,
}

#[cw_serde]
pub struct AnonymizedDataResponse {

    /* define parameters */

    pub data: String,
}
