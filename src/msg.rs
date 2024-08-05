use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};
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
    pub shit_rate: Decimal, // # of tokens needed to recieve 1 SHITMOS
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Entry point to participate in shit-strap
    ShitStrap { shit: AssetUnchecked },
    /// Admin function to set full-of-shit status to on
    Flush {},
    /// Cw20 Entry Point
    Receive(Cw20ReceiveMsg),
    /// Only addr that sent shit-strap into being full of shit can call this function to claim any excess funds.
    RefundShitter{}
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
    #[returns(Uint128)]
    /// Current amount of shit value that has been deposited in the shit-strap.
    /// Can be used to calculate how much more is needed for a full-of-shit status.
    ShitPile {},
    #[returns(bool)]
    FullOfShit {},
    #[returns(Option<Decimal>)]
    ShitRate{asset: String,}

}


#[cw_serde]
pub struct AssetUnchecked {
    pub denom: UncheckedDenom,
    pub amount: Uint128,
}
