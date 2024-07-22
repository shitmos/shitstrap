use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;
use cw_denom::UncheckedDenom;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub accepted: Vec<PossibleShit>,
    pub cutoff: Uint128, // desired value to be bootstrapped. Once reached, no more deposits are possible
    pub shitmos: String, // SHITMOS token address
}

#[cw_serde]
pub struct PossibleShit {
    pub token: UncheckedDenom,
    pub shit_rate: Uint128, // # of tokens needed to recieve 1 SHITMOS
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Entry point to participate in shit-strap
    ShitStrap { shit: AssetUnchecked },
    /// Admin function to set full-of-shit status to on
    Flush {},
    /// Cw20 Entry Point
    Receive(Cw20ReceiveMsg),
}

#[cw_serde]
pub enum ReceiveMsg {
    ShitStrap { shit_strapper: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns max possible deposit value for a shit-strap instance
    #[returns(Uint128)]
    Cutoff {},
}

#[cw_serde]
pub struct AssetUnchecked {
    pub denom: UncheckedDenom,
    pub amount: Uint128,
}
