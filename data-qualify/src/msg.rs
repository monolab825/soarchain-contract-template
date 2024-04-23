use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::types::Data;

#[cw_serde]
pub struct InstantiateMsg {
    // You can initialize any state or perform setup logic here
    pub data: Data,
}

// Define your custom struct for messages
#[cw_serde]
pub enum ExecuteMsg {
    QualifyData { 
        /* define parameters */    
        data: Data 
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

    #[returns(DataQualificationResultResponse)]
    GetDataQualificationResult { 
        /* define parameters */
    },

    #[returns(QualifiedDataResponse)]
    GetQualifiedData { 
        /* define parameters */
    },
    
}

// Define a custom struct for each query response

#[cw_serde]
pub struct DataQualificationResultResponse {

    /* define parameters */

    pub qualified: bool,
}

#[cw_serde]
pub struct QualifiedDataResponse {

    /* define parameters */

    pub data: Data,
}
