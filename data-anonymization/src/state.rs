use serde::{Deserialize, Serialize};
use cw_storage_plus::Item;

// Define your custom struct to represent the state of your contract

#[derive(Serialize, Deserialize)]
pub struct State {
    pub owner: String,
    pub json_data: String,
}

impl State {
    pub fn new() -> Self {
        State {
            owner: String::new(),
            json_data: String::new()
        }
    }
}


pub const STATE: Item<State> = Item::new("state");