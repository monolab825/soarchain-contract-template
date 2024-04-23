use serde::{Deserialize, Serialize};
use crate::types::Data;
use cw_storage_plus::Item;

// Define your custom struct to represent the state of your contract

#[derive(Serialize, Deserialize)]
pub struct State {
    pub owner: String,
    pub json_data: Data,
}

impl State {
    pub fn new() -> Self {
        State {
            owner: String::new(),
            json_data: Data {
                data_info:  crate::types::DataInfo { data_details: ({
                    crate::types::DataDetails { 
                            accelerometer: ({
                                crate::types::GeographicInfo { 
                                    x: 0u64, 
                                    y: 0u64, 
                                    z: 0u64 
                                }
                            }), 
                            gyroscope: ({
                                crate::types::GeographicInfo { 
                                    x: 0u64, 
                                    y: 0u64, 
                                    z: 0u64 
                                }
                            }), 
                            magnetometer: ({
                                crate::types::GeographicInfo { 
                                    x: 0u64, 
                                    y: 0u64, 
                                    z: 0u64 
                                }
                            }), 
                            location: ({
                                crate::types::LocationInfo { 
                                    lat: 0u64, 
                                    lng: 0u64 
                                }
                            }), 
                            trip: String::new(),
                            contract: String::new(),
                            vehicle_info: ({
                                crate::types::VehicleInfo { 
                                    load_pct: 0u64, 
                                    temp: 0u64, 
                                    rpm: 0u64, 
                                    vss: 0u64, 
                                    iat: 0u64, 
                                    maf: 0u64, 
                                    throttlepo: 0u64, 
                                    runtm: 0u64, 
                                    fli: 0u64, 
                                    baro: 0u64, 
                                    load_abs: 0u64, 
                                    fuel_rate: 0u64, 
                                    odometer: 0u64 
                                }
                            }) 
                        }
                    }) 
                },
                pubkey: String::new(),
                sign: String::new()
            },
        }
    }
}

pub const STATE: Item<State> = Item::new("state");