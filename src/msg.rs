use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;
use cw_denom::UncheckedDenom;

#[cw_serde]
pub struct InstantiateMsg {
    /// owner of the shit strap
    pub owner: String,
    /// a list of possible accepted assets
    pub accepted: Vec<PossibleShit>,
    /// Desired cutoff points for shitstrap. Once reached, no more deposits are possible.
    pub cutoff: Uint128,
    /// SHITMOS token address
    pub shitmos: String,
}

#[cw_serde]
pub struct PossibleShit {
    pub token: UncheckedDenom,
    /// Atomic unit value for conversion ratio with shitmos.\
    /// This contract defaults to 6 decimal places. *(1000000 == 1:1 coversion ratio)*
    pub shit_rate: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Entry point to participate in shit-strap
    ShitStrap { shit: AssetUnchecked },
    /// Admin function to set full-of-shit status to on. *(used for emergencies or early cutoff)*
    Flush {},
    /// Cw20 Entry Point
    Receive(Cw20ReceiveMsg),
    /// Refunds anyone that was the last one to shitstrap, and sent excess funds.
    RefundShitter {},
}

#[cw_serde]
pub enum ReceiveMsg {
    /// Manually register an address for a shit strap when sending cw20 tokens.
    /// This can be a different address than the sender, if desired.
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
    /// Query if the shit strap contract is no longer active
    FullOfShit {},
    /// Query the shit conversation ratio for a given asset
    #[returns(Option<Uint128>)]
    ShitRate { asset: String },
}

#[cw_serde]
pub struct AssetUnchecked {
    pub denom: UncheckedDenom,
    pub amount: Uint128,
}

impl AssetUnchecked {
    pub fn from_native(denom: &str, amount: u128) -> Self {
        AssetUnchecked {
            denom: UncheckedDenom::Native(denom.into()),
            amount: amount.into(),
        }
    }
}

impl PossibleShit {
    pub fn native_denom(native_denom: &str, shit_rate: u128) -> Self {
        PossibleShit {
            token: UncheckedDenom::Native(native_denom.into()),
            shit_rate: Uint128::new(shit_rate),
        }
    }
    pub fn native_cw20(native_coin: &str, shit_rate: u128) -> Self {
        PossibleShit {
            token: UncheckedDenom::Cw20(native_coin.into()),
            shit_rate: Uint128::new(shit_rate),
        }
    }
}
