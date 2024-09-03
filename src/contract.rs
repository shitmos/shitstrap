#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, from_json, to_json_binary, Addr, Attribute, Binary, CosmosMsg, Decimal, Deps, DepsMut,
    Empty, Env, Fraction, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use cw_denom::{CheckedDenom, UncheckedDenom};

use crate::error::ContractError;
use crate::msg::{AssetUnchecked, ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg};
use crate::state::{
    Config, ATOMINC_DECIMALS, CONFIG, CURRENT_SHITSTRAP_VALUE, REFUND_SHIT, SHITSTRAP_STATE,
};

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
    // set owner
    let owner = deps.api.addr_validate(&msg.owner)?;

    // save contract instance config
    CONFIG.save(
        deps.storage,
        &Config {
            owner,
            accepted: msg.accepted,
            cutoff: msg.cutoff * Uint128::from(1_000_000u64), // moves 6 decimal places for minimal denoms
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
    let mut attrs = vec![];
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
        let shit_value = shit.amount * Decimal::from_atomics(matched.shit_rate, ATOMINC_DECIMALS)?;
        let received_denom = matched.clone().token.into_checked(deps.as_ref())?;

        // if new_val > or = cutoff,
        // any excess funds sent are able to be claimed by sender.
        let new_val = shit_value + current_shit_value.clone();
        let cutoff = config.cutoff.clone();

        if new_val.clone() >= cutoff.clone() {
            // new_sv = sv + current - cutoff
            // return assets = sv - new_sv / shit_rate
            // gets the amount of tokens sent after cutoff limit
            let overflow = new_val.clone() - cutoff.clone();

            // reverse the shit rate calculation to get the exact # of tokens to return
            let return_to_shitter_amnt: Uint128 = overflow
                * Decimal::from_atomics(matched.shit_rate, ATOMINC_DECIMALS)?
                    .inv()
                    .expect("ahh");

            // for each token sent in shitstrap, transfer to dao
            for owned in
                SHITSTRAP_STATE.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            {
                let mut _amnt = 0u128;
                let tokens = owned?;

                if tokens.0 == received_denom.to_string() {
                    _amnt = (tokens.1 + return_to_shitter_amnt).u128()
                } else {
                    _amnt = tokens.1.u128()
                }
                // send new token amount to admin
                let shitstrap_dao = shitstrap_dao(
                    received_denom.clone(),
                    tokens.0,
                    _amnt.into(),
                    config.owner.clone(),
                )?;
                
                // msg attributes for indexing
                let attr1 = Attribute::new("dao_shitstrap_amount", shit_value.to_string());
                let attr2 = Attribute::new("dao_shitstrap_recieved", received_denom.to_string());
                attrs.extend(vec![attr1, attr2]);
                
                // println!("overflow: {:#?}", overflow);
                // println!("return_to_shitter_amnt: {:#?}", return_to_shitter_amnt);
                // println!("shitstrap_dao: {:#?}", shitstrap_dao);
                msgs.push(shitstrap_dao);
            }

            // form return msgs
            let msg: CosmosMsg = match received_denom.clone() {
                CheckedDenom::Native(shit) => CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
                    to_address: shit_strapper.to_string(),
                    amount: coins(return_to_shitter_amnt.into(), shit),
                }),
                CheckedDenom::Cw20(shit) => CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                    contract_addr: shit.to_string(),
                    msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                        recipient: shit_strapper.to_string(),
                        amount: return_to_shitter_amnt,
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

        // update internal shitstrap value, & save asset-specific shit strap state.

        let add_count = |prev: Option<Uint128>| -> StdResult<Uint128> {
            prev.unwrap_or_default()
                .checked_add(Uint128::new(shit.amount.u128()))
                .map_err(StdError::overflow)
        };
        CURRENT_SHITSTRAP_VALUE.save(deps.storage, &new_val)?;
        SHITSTRAP_STATE.update(deps.storage, received_denom.to_string(), &add_count)?;

        // push msg to response
        msgs.push(send_shitmos)
    } else {
        return Err(ContractError::WrongShit {});
    }

    Ok(Response::new().add_messages(msgs).add_attributes(attrs))
}

pub fn execute_flush(deps: DepsMut, sender: Addr) -> Result<Response, ContractError> {
    // only owner can call this
    if sender != CONFIG.load(deps.storage)?.owner {
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
    addr: String,
    amount: Uint128,
    dao: Addr,
) -> Result<CosmosMsg, StdError> {
    // send tokens to admin
    match recieved {
        CheckedDenom::Native(_) => Ok(CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
            to_address: dao.to_string(),
            amount: coins(amount.into(), addr),
        })),
        CheckedDenom::Cw20(_) => Ok(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: addr.to_string(),
            msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                recipient: dao.to_string(),
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
