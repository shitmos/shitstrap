use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_ownable::cw_ownable_execute;

use cw_shitstrap::msg::InstantiateMsg as ShitstrapInstantiateMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
    pub shitstrap_id: u64,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    /// Instantiates a new vesting contract that is funded by a native token.
    CreateNativeShitStrapContract {
        instantiate_msg: ShitstrapInstantiateMsg,
        label: String,
    },

    /// Callable only by the current owner. Updates the code ID used
    /// while instantiating vesting contracts.
    UpdateCodeId { shitstrap_code_id: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns list of all vesting payment contracts
    #[returns(Vec<crate::state::ShitstrapContract>)]
    ListShitstrapContracts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Returns list of all vesting payment contracts in reverse
    #[returns(Vec<crate::state::ShitstrapContract>)]
    ListShitstrapContractsReverse {
        start_before: Option<String>,
        limit: Option<u32>,
    },
    /// Returns list of all vesting payment contracts by who instantiated them
    #[returns(Vec<crate::state::ShitstrapContract>)]
    ListShitstrapContractsByInstantiator {
        instantiator: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Returns list of all vesting payment contracts by who instantiated them in reverse
    #[returns(Vec<crate::state::ShitstrapContract>)]
    ListShitstrapContractsByInstantiatorReverse {
        instantiator: String,
        start_before: Option<String>,
        limit: Option<u32>,
    },
    /// Returns list of all vesting payment contracts by recipient
    #[returns(Vec<crate::state::ShitstrapContract>)]
    ListShitstrapContractsByToken {
        recipient: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Returns list of all vesting payment contracts by recipient in reverse
    #[returns(Vec<crate::state::ShitstrapContract>)]
    ListShitstrapContractsByTokenReverse {
        recipient: String,
        start_before: Option<String>,
        limit: Option<u32>,
    },
    /// Returns info about the contract ownership, if set
    #[returns(::cw_ownable::Ownership<::cosmwasm_std::Addr>)]
    Ownership {},

    /// Returns the code ID currently being used to instantiate vesting contracts.
    #[returns(::std::primitive::u64)]
    CodeId {},
}
