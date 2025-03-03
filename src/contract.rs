#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, from_json, to_json_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Empty, Env,
    MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use cw_denom::{CheckedDenom, UncheckedDenom};

use crate::error::ContractError;
use crate::msg::{AssetUnchecked, ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg};
use crate::state::{Config, CONFIG, CURRENT_SHITSTRAP_VALUE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-shit-strap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // set admin
    let admin = deps.api.addr_validate(&msg.owner)?;

    // save contract instance config
    CONFIG.save(
        deps.storage,
        &Config {
            admin,
            accepted: msg.accepted,
            cutoff: msg.cutoff,
            shitmos_addr: msg.shitmos,
            full_of_shit: false,
        },
    )?;

    // set shit-strapped value to 0
    CURRENT_SHITSTRAP_VALUE.save(deps.storage, &Uint128::zero())?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ShitStrap { shit } => execute_shit_strap(deps, shit, info.sender),
        ExecuteMsg::Flush {} => execute_flush(deps, info.sender),
        ExecuteMsg::Receive(cw20_msg) => receive_cw20_message(deps, info, cw20_msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Cutoff {} => to_json_binary(&CONFIG.load(deps.storage)?.cutoff),
    }
}

/// Entry point to particpate in shitstrap
pub fn execute_shit_strap(
    deps: DepsMut,
    shit: AssetUnchecked,
    sender: Addr,
) -> Result<Response, ContractError> {
    let mut msgs = vec![];
    let mut config = CONFIG.load(deps.storage)?;
    let current_shit_value = CURRENT_SHITSTRAP_VALUE.load(deps.storage)?;

    if config.full_of_shit {
        return Err(ContractError::FullOfShit {});
    }

    // verify sent shit is one of the accepted shits
    if let Some(matched) = config
        .accepted
        .clone()
        .into_iter()
        .find(|c| c.token == shit.denom)
    {
        // defines conversion rate for accepted shit to SHITMOS
        let mut shit_value = matched.shit_rate * shit.amount;
        let received_denom = matched.clone().token.into_checked(deps.as_ref())?;

        // if new value is greater than cutoff,
        // define value shit-strapper recieves as value that reaches cutoff limit.
        // any excess funds sent are returned.
        let new_val = shit_value + current_shit_value.clone();
        let cutoff = CONFIG.load(deps.storage)?.cutoff;

        if new_val.clone() > cutoff.clone() {
            // new_sv = sv + current - cutoff
            // return assets = sv - new_sv / shit_rate
            let old_shit_value = shit_value;
            shit_value = new_val.clone() - cutoff.clone();
            let shit_2_return = (old_shit_value - shit_value) / matched.shit_rate;

            // send new token amount to admin
            let shitstrap_dao = shitstrap_dao(received_denom.clone(), sender.clone(), shit_value)?;
            msgs.push(shitstrap_dao);

            // form return msgs
            let msg: CosmosMsg = match received_denom.clone() {
                CheckedDenom::Native(shit) => CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
                    to_address: sender.to_string(),
                    amount: coins(shit_2_return.into(), shit),
                }),
                CheckedDenom::Cw20(shit) => CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                    contract_addr: shit.to_string(),
                    msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                        recipient: sender.to_string(),
                        amount: shit_2_return,
                    })?,
                    funds: vec![],
                }),
            };
            // push msg to response
            msgs.push(msg);

            // shit-strap is now complete.
            config.full_of_shit = true;
            CONFIG.save(deps.storage, &config)?;
        }
        // form SHITMOS transfer msg.
        let send_shitmos: CosmosMsg<Empty> = CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
            to_address: sender.to_string(),
            amount: coins(shit_value.into(), config.shitmos_addr),
        });

        // push msg to response
        msgs.push(send_shitmos)
    } else {
        return Err(ContractError::WrongShit {});
    }

    Ok(Response::new().add_messages(msgs))
}

pub fn execute_flush(deps: DepsMut, sender: Addr) -> Result<Response, ContractError> {
    // only owner can call this
    if sender != CONFIG.load(deps.storage)?.admin {
        return Err(ContractError::Unauthorized {});
    }
    let mut config = CONFIG.load(deps.storage)?;
    config.full_of_shit = true;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "value"))
}

fn receive_cw20_message(
    deps: DepsMut,
    info: MessageInfo,
    msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_json(&msg.msg)? {
        ReceiveMsg::ShitStrap { shit_strapper } => {
            let sender = deps.api.addr_validate(&shit_strapper)?;
            execute_shit_strap(
                deps,
                AssetUnchecked {
                    denom: UncheckedDenom::Cw20(info.sender.to_string()),
                    amount: msg.amount,
                },
                sender,
            )
        }
    }
}

fn shitstrap_dao(
    recieved: CheckedDenom,
    sender: Addr,
    amount: Uint128,
) -> Result<CosmosMsg, StdError> {
    // send tokens to admin
    match recieved {
        CheckedDenom::Native(shiet) => Ok(CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
            to_address: sender.to_string(),
            amount: coins(amount.into(), shiet),
        })),
        CheckedDenom::Cw20(shiet) => Ok(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: shiet.to_string(),
            msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                recipient: sender.to_string(),
                amount: amount.into(),
            })?,
            funds: vec![],
        })),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_init() {

    }
}
