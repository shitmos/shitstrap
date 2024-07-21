use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

use crate::msg::PossibleShit;

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub accepted: Vec<PossibleShit>,
    pub cutoff: Uint128,
    pub shitmos_addr: String,
    pub full_of_shit: bool, // once cutoff is reached, full of shit set to true
}

pub const CONFIG: Item<Config> = Item::new("cfg");
pub const CURRENT_SHITSTRAP_VALUE: Item<Uint128> = Item::new("csv");
