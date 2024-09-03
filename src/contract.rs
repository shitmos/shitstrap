#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, from_json, to_json_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, Fraction,
    MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use cw_denom::{CheckedDenom, UncheckedDenom};

use crate::error::ContractError;
use crate::msg::{AssetUnchecked, ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg};
use crate::state::{Config, CONFIG, CURRENT_SHITSTRAP_VALUE, REFUND_SHIT};

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

fn refund_shitter(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut msg = vec![];
    if let Some(refund) = REFUND_SHIT.may_load(deps.storage, info.sender)? {
        msg.push(refund);
        REFUND_SHIT.clear(deps.storage);
    } else {
        return Err(ContractError::FullOfShit {});
    }
    Ok(Response::new().add_messages(msg))
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ShitStrap { shit } => execute_shit_strap(deps, info.clone(), shit, info.sender),
        ExecuteMsg::Flush {} => execute_flush(deps, info.sender),
        ExecuteMsg::Receive(cw20_msg) => receive_cw20_message(deps, info, cw20_msg),
        ExecuteMsg::RefundShitter {} => refund_shitter(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Cutoff {} => to_json_binary(&CONFIG.load(deps.storage)?.cutoff),
        QueryMsg::ShitPile {} => to_json_binary(&CURRENT_SHITSTRAP_VALUE.load(deps.storage)?),
        QueryMsg::FullOfShit {} => to_json_binary(&CONFIG.load(deps.storage)?.full_of_shit),
        QueryMsg::ShitRate { asset } => to_json_binary(
            &CONFIG
                .load(deps.storage)?
                .accepted
                .into_iter()
                .map(|c| match c.token {
                    UncheckedDenom::Native(n) => {
                        if n == asset.clone() {
                            Some(c.shit_rate)
                        } else {
                            None
                        }
                    }
                    UncheckedDenom::Cw20(cw) => {
                        if cw == asset.clone() {
                            Some(c.shit_rate)
                        } else {
                            None
                        }
                    }
                })
                .into_iter()
                .flatten()
                .next(),
        ),
    }
}

/// Entry point to particpate in shitstrap
pub fn execute_shit_strap(
    deps: DepsMut,
    info: MessageInfo,
    shit: AssetUnchecked,
    shit_strapper: Addr,
) -> Result<Response, ContractError> {
    let mut msgs = vec![];
    let mut config = CONFIG.load(deps.storage)?;
    let current_shit_value = CURRENT_SHITSTRAP_VALUE.load(deps.storage)?;

    if config.full_of_shit {
        return Err(ContractError::FullOfShit {});
    }

    if let Some(matched) = config
        .accepted
        .clone()
        .into_iter()
        .find(|c| c.token == shit.denom)
    {
        match matched.token.clone() {
            UncheckedDenom::Native(t) => {
                if !info
                    .funds
                    .into_iter()
                    .find(|c| c.denom == t)
                    .is_some_and(|c| c.amount == shit.amount)
                {
                    return Err(ContractError::DidntSendShit {});
                }
            }
            UncheckedDenom::Cw20(c) => {
                // info.sender should always be one of the accepted tokens
                if info.sender != c {
                    return Err(ContractError::ShittyCw20 {});
                }
            }
        }
        // defines conversion rate for accepted shit to SHITMOS
        let shit_value = shit.amount * matched.shit_rate;
        let received_denom = matched.clone().token.into_checked(deps.as_ref())?;

        // if new value is greater than cutoff,
        // define value shit-strapper recieves as value that reaches cutoff limit.
        // any excess funds sent are returned.
        let new_val = shit_value + current_shit_value.clone();
        let cutoff = config.cutoff.clone();

        if new_val.clone() >= cutoff.clone() {
            // new_sv = sv + current - cutoff
            // return assets = sv - new_sv / shit_rate
            // gets the amount of tokens sent after cutoff limit
            let cutoff_shit_value = new_val.clone() - cutoff.clone();

            let shit_2_return: Uint128 = cutoff_shit_value * matched.shit_rate.inv().expect("ahh");

            // send new token amount to admin
            let shitstrap_dao =
                shitstrap_dao(received_denom.clone(), shit_strapper.clone(), shit_value)?;
            msgs.push(shitstrap_dao);

            // form return msgs
            let msg: CosmosMsg = match received_denom.clone() {
                CheckedDenom::Native(shit) => CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
                    to_address: shit_strapper.to_string(),
                    amount: coins(shit_2_return.into(), shit),
                }),
                CheckedDenom::Cw20(shit) => CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                    contract_addr: shit.to_string(),
                    msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                        recipient: shit_strapper.to_string(),
                        amount: shit_2_return,
                    })?,
                    funds: vec![],
                }),
            };
            // push msg to response
            REFUND_SHIT.save(deps.storage, shit_strapper.clone(), &msg)?;

            // shit-strap is now complete.
            config.full_of_shit = true;
            CONFIG.save(deps.storage, &config)?;
        }
        // form SHITMOS transfer msg.
        let send_shitmos: CosmosMsg<Empty> = CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
            to_address: shit_strapper.to_string(),
            amount: coins(shit_value.into(), config.shitmos_addr),
        });

        CURRENT_SHITSTRAP_VALUE.save(deps.storage, &new_val)?;
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
        return Err(ContractError::ShittyAuthorization {});
    }
    let mut config = CONFIG.load(deps.storage)?;
    config.full_of_shit = true;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("flusher", sender.to_string()))
}

fn receive_cw20_message(
    deps: DepsMut,
    info: MessageInfo,
    msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_json(&msg.msg)? {
        // only cw20 can call cw20 entry point.
        // we set the denom for the cw20 as info.sender,
        // which will result in error if any addr other than accepted cw20 makes call.
        ReceiveMsg::ShitStrap { shit_strapper } => {
            let sender = deps.api.addr_validate(&shit_strapper)?;
            execute_shit_strap(
                deps,
                info.clone(),
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
    pub fn test_init() {}
}
