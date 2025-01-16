#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Reply, Response, StdResult,
    SubMsg, WasmMsg,
};
use cosmwasm_std::{Addr, Coin};

use cw2::set_contract_version;
use cw_shitstrap::{
    msg::{InstantiateMsg as ShitstrapInstantiateMsg, QueryMsg as ShitstrapQueryMsg},
    state::Config as ShitstrapConfig,
};
use cw_storage_plus::Bound;
use cw_utils::parse_reply_instantiate_data;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    shitstrap_contracts, ShitstrapContract, SHITSTRAP_CODE_ID, TMP_INSTANTIATOR_INFO,
};

pub(crate) const CONTRACT_NAME: &str = "crates.io:cw-shitstrap-factory";
pub(crate) const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const INSTANTIATE_CONTRACT_REPLY_ID: u64 = 0;
pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 50;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw_ownable::initialize_owner(deps.storage, deps.api, msg.owner.as_deref())?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    SHITSTRAP_CODE_ID.save(deps.storage, &msg.shitstrap_id)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("creator", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateNativeShitStrapContract {
            instantiate_msg,
            label,
        } => execute_instantiate_native_shitstrap_contract(deps, info, instantiate_msg, label),
        ExecuteMsg::UpdateOwnership(action) => execute_update_owner(deps, info, env, action),
        ExecuteMsg::UpdateCodeId { shitstrap_code_id } => {
            execute_update_code_id(deps, info, shitstrap_code_id)
        }
    }
}

pub fn execute_instantiate_native_shitstrap_contract(
    deps: DepsMut,
    info: MessageInfo,
    instantiate_msg: ShitstrapInstantiateMsg,
    label: String,
) -> Result<Response, ContractError> {
    // Save instantiator info for use in reply
    TMP_INSTANTIATOR_INFO.save(deps.storage, &info.sender)?;

    instantiate_contract(deps, info.sender, Some(info.funds), instantiate_msg, label)
}

/// `sender` here refers to the initiator of the shistrap, not the
/// literal sender of the message. Practically speaking, this means
/// that it should be set to the sender of the cw20's being vested,
/// and not the cw20 contract when dealing with non-native shistrap.
pub fn instantiate_contract(
    deps: DepsMut,
    sender: Addr,
    funds: Option<Vec<Coin>>,
    instantiate_msg: ShitstrapInstantiateMsg,
    label: String,
) -> Result<Response, ContractError> {
    // Check sender is contract owner if set
    let ownership = cw_ownable::get_ownership(deps.storage)?;
    if ownership
        .owner
        .as_ref()
        .is_some_and(|owner| *owner != sender)
    {
        return Err(ContractError::Unauthorized {});
    }

    let code_id = SHITSTRAP_CODE_ID.load(deps.storage)?;

    // Instantiate the specified contract with owner as the admin.
    let instantiate = WasmMsg::Instantiate {
        admin: instantiate_msg.owner.clone(),
        code_id,
        msg: to_json_binary(&instantiate_msg)?,
        funds: funds.unwrap_or_default(),
        label,
    };

    let msg = SubMsg::reply_on_success(instantiate, INSTANTIATE_CONTRACT_REPLY_ID);

    Ok(Response::default()
        .add_attribute("action", "instantiate_cw_shistrap")
        .add_submessage(msg))
}

pub fn execute_update_owner(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    action: cw_ownable::Action,
) -> Result<Response, ContractError> {
    let ownership = cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
    Ok(Response::default().add_attributes(ownership.into_attributes()))
}

pub fn execute_update_code_id(
    deps: DepsMut,
    info: MessageInfo,
    shistrap_code_id: u64,
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;
    SHITSTRAP_CODE_ID.save(deps.storage, &shistrap_code_id)?;
    Ok(Response::default()
        .add_attribute("action", "update_code_id")
        .add_attribute("shistrap_code_id", shistrap_code_id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ListShitstrapContracts { start_after, limit } => {
            let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
            let start = start_after.as_deref().map(Bound::exclusive);

            let res: Vec<ShitstrapContract> = shitstrap_contracts()
                .range(deps.storage, start, None, Order::Ascending)
                .take(limit)
                .flat_map(|vc| Ok::<ShitstrapContract, ContractError>(vc?.1))
                .collect();

            Ok(to_json_binary(&res)?)
        }
        QueryMsg::ListShitstrapContractsReverse {
            start_before,
            limit,
        } => {
            let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
            let start = start_before.as_deref().map(Bound::exclusive);

            let res: Vec<ShitstrapContract> = shitstrap_contracts()
                .range(deps.storage, None, start, Order::Descending)
                .take(limit)
                .flat_map(|vc| Ok::<ShitstrapContract, ContractError>(vc?.1))
                .collect();

            Ok(to_json_binary(&res)?)
        }
        QueryMsg::ListShitstrapContractsByInstantiator {
            instantiator,
            start_after,
            limit,
        } => {
            let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
            let start = start_after.map(Bound::<String>::exclusive);

            // Validate owner address
            deps.api.addr_validate(&instantiator)?;

            let res: Vec<ShitstrapContract> = shitstrap_contracts()
                .idx
                .instantiator
                .prefix(instantiator)
                .range(deps.storage, start, None, Order::Ascending)
                .take(limit)
                .flat_map(|vc| Ok::<ShitstrapContract, ContractError>(vc?.1))
                .collect();

            Ok(to_json_binary(&res)?)
        }
        QueryMsg::ListShitstrapContractsByInstantiatorReverse {
            instantiator,
            start_before,
            limit,
        } => {
            let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
            let start = start_before.map(Bound::<String>::exclusive);

            // Validate owner address
            deps.api.addr_validate(&instantiator)?;

            let res: Vec<ShitstrapContract> = shitstrap_contracts()
                .idx
                .instantiator
                .prefix(instantiator)
                .range(deps.storage, None, start, Order::Descending)
                .take(limit)
                .flat_map(|vc| Ok::<ShitstrapContract, ContractError>(vc?.1))
                .collect();

            Ok(to_json_binary(&res)?)
        }
        QueryMsg::ListShitstrapContractsByToken {
            recipient,
            start_after,
            limit,
        } => {
            let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
            let start = start_after.map(Bound::<String>::exclusive);

            // Validate recipient address
            deps.api.addr_validate(&recipient)?;

            let res: Vec<ShitstrapContract> = shitstrap_contracts()
                .idx
                .shit
                .prefix(recipient)
                .range(deps.storage, start, None, Order::Ascending)
                .take(limit)
                .flat_map(|vc| Ok::<ShitstrapContract, ContractError>(vc?.1))
                .collect();

            Ok(to_json_binary(&res)?)
        }
        QueryMsg::ListShitstrapContractsByTokenReverse {
            recipient,
            start_before,
            limit,
        } => {
            let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
            let start = start_before.map(Bound::<String>::exclusive);

            // Validate recipient address
            deps.api.addr_validate(&recipient)?;

            let res: Vec<ShitstrapContract> = shitstrap_contracts()
                .idx
                .shit
                .prefix(recipient)
                .range(deps.storage, None, start, Order::Descending)
                .take(limit)
                .flat_map(|vc| Ok::<ShitstrapContract, ContractError>(vc?.1))
                .collect();

            Ok(to_json_binary(&res)?)
        }
        QueryMsg::Ownership {} => to_json_binary(&cw_ownable::get_ownership(deps.storage)?),
        QueryMsg::CodeId {} => to_json_binary(&SHITSTRAP_CODE_ID.load(deps.storage)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        INSTANTIATE_CONTRACT_REPLY_ID => {
            let res = parse_reply_instantiate_data(msg)?;
            let contract_addr = deps.api.addr_validate(&res.contract_address)?;

            // Query new shistrap payment contract for info
            let shit_strap: ShitstrapConfig = deps
                .querier
                .query_wasm_smart(contract_addr.clone(), &ShitstrapQueryMsg::Config {})?;

            let instantiator = TMP_INSTANTIATOR_INFO.load(deps.storage)?;

            // Save shistrap contract payment info
            shitstrap_contracts().save(
                deps.storage,
                contract_addr.as_ref(),
                &ShitstrapContract {
                    instantiator: instantiator.to_string(),
                    shit: shit_strap.shitmos_addr.to_string(),
                    contract: contract_addr.to_string(),
                },
            )?;

            // Clear tmp instatiator info
            TMP_INSTANTIATOR_INFO.remove(deps.storage);

            Ok(Response::default().add_attribute("new_shitstrap_contract", contract_addr))
        }
        _ => Err(ContractError::UnknownReplyId { id: msg.id }),
    }
}
