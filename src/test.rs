use cosmwasm_std::{coin, to_json_binary, Addr, Decimal, Empty, Uint128};
use cw20::{Cw20Coin, Cw20ReceiveMsg};
use cw20_base::msg::InstantiateMsg as Cw20Init;
use cw_multi_test::{App, AppResponse, BankSudo, Contract, ContractWrapper, Executor, SudoMsg};
use cw_orch::anyhow::{self, Error};

use crate::{
    msg::{AssetUnchecked, InstantiateMsg, PossibleShit, ReceiveMsg},
    state::ATOMINC_DECIMALS,
    ContractError,
};

pub const DEFAULT_BALANCE: u128 = 1_000_000_000;
pub const DEFAULT_CW20: &str = "contract0";
pub const OWNER: &str = "owner";
pub const SHITTER1: &str = "shitter1";
pub const SHITTER2: &str = "shitter2";
pub const SHITTER3: &str = "shitter3";
pub const SHITTER4: &str = "shitter4";

fn cw20_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}

fn shitstrap_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

fn instantiate_w_cw20(
    mut app: App,
    shit_id: u64,
    init: InstantiateMsg,
    cw20_id: u64,
    cw20: Cw20Init,
) -> ShitSuite {
    // init cw20
    let cw20 = app
        .instantiate_contract(
            cw20_id,
            Addr::unchecked(OWNER),
            &cw20,
            &[],
            "cw20",
            Some(OWNER.into()),
        )
        .unwrap();
    // init shitstrap
    let shitstrap = app
        .instantiate_contract(
            shit_id,
            Addr::unchecked(OWNER),
            &init,
            &[],
            "shitstrap",
            Some(OWNER.into()),
        )
        .unwrap();

    ShitSuite {
        app,
        shitstrap,
        cw20,
    }
}

fn default_init(possible: Vec<PossibleShit>, cutoff: u128) -> ShitSuite {
    // create simulation environment
    let mut app = App::default();
    let cw20_id = app.store_code(cw20_contract());
    let shitstrap_id = app.store_code(shitstrap_contract());

    // create cw20
    let cw20_init = Cw20Init {
        name: "poo".into(),
        symbol: "POO".into(),
        decimals: 6,
        initial_balances: vec![
            Cw20Coin {
                address: SHITTER2.to_string(),
                amount: 1000u128.into(),
            },
            Cw20Coin {
                address: SHITTER3.to_string(),
                amount: 1000u128.into(),
            },
        ],
        mint: None,
        marketing: None,
    };
    // create shitstrap
    let init = InstantiateMsg {
        owner: OWNER.into(),
        accepted: possible,
        cutoff: cutoff.into(),
        shitmos: "ushit".into(),
    };

    // instantiate contract with cw20
    let suite = instantiate_w_cw20(app, shitstrap_id, init, cw20_id, cw20_init.clone());
    // return suite
    suite
}

pub struct ShitSuite {
    pub app: App,
    pub shitstrap: Addr,
    pub cw20: Addr,
}

impl ShitSuite {
    /// funds testing accounts with default balance
    fn setup_default_funds(&mut self, shitstrap: Addr) -> cw_orch::anyhow::Result<(), Error> {
        self.app
            .sudo(SudoMsg::Bank(BankSudo::Mint {
                to_address: OWNER.into(),
                amount: vec![coin(1_000_000_000u128, "uatom")],
            }))
            .unwrap();
        self.app
            .sudo(SudoMsg::Bank(BankSudo::Mint {
                to_address: SHITTER1.into(),
                amount: vec![coin(1_000_000_000u128, "uatom")],
            }))
            .unwrap();
        self.app
            .sudo(SudoMsg::Bank(BankSudo::Mint {
                to_address: SHITTER2.into(),
                amount: vec![coin(1_000_000_000u128, "usilk")],
            }))
            .unwrap();
        // fund shitstrap with 1 million shit
        self.app
            .sudo(SudoMsg::Bank(BankSudo::Mint {
                to_address: shitstrap.to_string(),
                amount: vec![coin(1_000_000_000_000u128, "ushit")],
            }))
            .unwrap();
        Ok(())
    }

    /// helper function to participates in shitstrap with c20 coin
    fn participate_cw20(
        &mut self,
        sender: &str,
        amount: u128,
        contract: &str,
    ) -> Result<AppResponse, Error> {
        self.app.execute_contract(
            Addr::unchecked(contract.to_string()),
            self.shitstrap.clone(),
            &crate::msg::ExecuteMsg::Receive(Cw20ReceiveMsg {
                sender: sender.into(),
                amount: amount.into(),
                msg: to_json_binary(&ReceiveMsg::ShitStrap {
                    shit_strapper: sender.to_string(),
                })?,
            }),
            &vec![],
        )
    }
    /// helper function to participate in shitstrap with native coin
    fn participate_native(
        &mut self,
        sender: &str,
        amount: u128,
        denom: &str,
    ) -> Result<AppResponse, Error> {
        self.app.execute_contract(
            Addr::unchecked(sender.to_string()),
            self.shitstrap.clone(),
            &crate::msg::ExecuteMsg::ShitStrap {
                shit: AssetUnchecked {
                    denom: cw_denom::UncheckedDenom::Native(denom.into()),
                    amount: amount.into(),
                },
            },
            &vec![coin(amount, denom)],
        )
    }
}

#[test]
fn test_shitstrap() -> cw_orch::anyhow::Result<(), Error> {
    // create default testing suite
    let mut shit = default_init(
        vec![PossibleShit::native_denom("uatom", 1_000_000u128)], // 1:1 ratio
        222u128,
    );
    // deposit 1 less than max
    let first_deposit = 221_000_000u128;
    let shitstrap = shit.shitstrap.clone();
    shit.setup_default_funds(shitstrap.clone())?;

    // error with wrong native token
    let err = shit
        .app
        .execute_contract(
            Addr::unchecked(OWNER.to_string()),
            shitstrap.clone(),
            &crate::msg::ExecuteMsg::ShitStrap {
                shit: AssetUnchecked::from_native("usilk", first_deposit),
            },
            &vec![coin(first_deposit, "uatom")],
        )
        .unwrap_err();
    assert_eq!(ContractError::WrongShit {}, err.downcast().unwrap());

    // error with wrong cw20 token
    let err = shit
        .participate_cw20(OWNER, first_deposit, "contract0")
        .unwrap_err();
    assert_eq!(ContractError::WrongShit {}, err.downcast().unwrap());

    // error without sending token
    let err = shit
        .app
        .execute_contract(
            Addr::unchecked(SHITTER1.to_string()),
            shitstrap.clone(),
            &crate::msg::ExecuteMsg::ShitStrap {
                shit: AssetUnchecked::from_native("uatom", first_deposit),
            },
            &vec![],
        )
        .unwrap_err();
    assert_eq!(ContractError::DidntSendShit {}, err.downcast().unwrap());

    // move forward in time
    let mut block = shit.app.block_info();
    block.height += 1;
    shit.app.set_block(block);

    // error with correct token, but less sent then specified
    shit.app
        .execute_contract(
            Addr::unchecked(SHITTER1.to_string()),
            shitstrap.clone(),
            &crate::msg::ExecuteMsg::ShitStrap {
                shit: AssetUnchecked::from_native("uatom", first_deposit),
            },
            &vec![coin(22, "uatom")],
        )
        .unwrap_err();

    // move forward in time
    let mut block = shit.app.block_info();
    block.height += 1;
    shit.app.set_block(block);

    let og_owner_bal = shit.app.wrap().query_all_balances(OWNER)?[0].clone();
    assert_eq!(og_owner_bal.amount, Uint128::from(DEFAULT_BALANCE));

    // participate in shitstrap with correct token
    shit.participate_native(SHITTER1, 221_000_000, "uatom")?;

    // confirm shit_rate is calculated correctly
    let res: Uint128 = shit
        .app
        .wrap()
        .query_wasm_smart(shitstrap.clone(), &crate::msg::QueryMsg::ShitPile {})?;
    assert_eq!(res, Uint128::new(first_deposit));

    // confirm new balance of shitstrap
    let balance = shit.app.wrap().query_balance(shitstrap.clone(), "uatom")?;
    let shit_rate: Option<Uint128> = shit.app.wrap().query_wasm_smart(
        shitstrap.clone(),
        &crate::msg::QueryMsg::ShitRate {
            asset: "uatom".to_string(),
        },
    )?;
    // calulate expected
    let calculated = balance.amount * Decimal::from_atomics(shit_rate.unwrap(), ATOMINC_DECIMALS)?;
    assert_eq!(calculated, Uint128::from(first_deposit));

    // shitstrap reaches limit
    shit.app.execute_contract(
        Addr::unchecked(SHITTER1.to_string()),
        shitstrap.clone(),
        &crate::msg::ExecuteMsg::ShitStrap {
            shit: AssetUnchecked::from_native("uatom", 2_000_000u128),
        },
        &vec![coin(2000000u128, "uatom")],
    )?;

    // move forward in time
    let mut block = shit.app.block_info();
    block.height += 1;
    shit.app.set_block(block);

    // confirm contract will not continue to shitstrap
    let res: bool = shit
        .app
        .wrap()
        .query_wasm_smart(shit.shitstrap, &crate::msg::QueryMsg::FullOfShit {})?;
    assert_eq!(res, true);

    // confirm balances
    let balance = shit.app.wrap().query_balance(shitstrap.clone(), "uatom")?;
    assert_eq!(balance.amount, Uint128::from(1_000_000u128)); // 1 token is waiting to be redeemed by last shit strapper
    let balance = shit.app.wrap().query_balance(SHITTER1, "uatom")?;
    assert_eq!(balance.amount, Uint128::from(777_000_000u128));
    let owner_bal = shit.app.wrap().query_all_balances(OWNER)?[0].clone();
    assert_eq!(owner_bal.amount, Uint128::from(1_222_000_000u128)); // owner received 222 ATOM

    // no more shitstrapping can commence
    let err = shit
        .app
        .execute_contract(
            Addr::unchecked(SHITTER1.to_string()),
            shitstrap.clone(),
            &crate::msg::ExecuteMsg::ShitStrap {
                shit: AssetUnchecked::from_native("uatom", 2_000_000u128),
            },
            &vec![coin(2_000_000u128, "uatom")],
        )
        .unwrap_err();
    assert_eq!(ContractError::FullOfShit {}, err.downcast().unwrap());

    // refund on shitstrapping occurs
    shit.app.execute_contract(
        Addr::unchecked(SHITTER1.to_string()),
        shitstrap.clone(),
        &crate::msg::ExecuteMsg::RefundShitter {},
        &[],
    )?;

    // move forward in time
    let mut block = shit.app.block_info();
    block.height += 1;
    shit.app.set_block(block);

    // should have 1 extra token sent back
    let balance = shit.app.wrap().query_balance(SHITTER1, "uatom")?;
    assert_eq!(balance.amount, Uint128::from(778_000_000u128));
    let balance = shit.app.wrap().query_balance(shitstrap.clone(), "uatom")?;
    assert_eq!(balance.amount, Uint128::zero()); // 1 token is waiting to be redeemed by last shit strapper

    Ok(())
}

#[test]
fn test_mult_participants_mult_possible_shit() -> cw_orch::anyhow::Result<(), Error> {
    // create testing suite
    let first_deposit = 100_000_000u128;
    let cw20_shit_ratio = 640000u64;
    let atom_shit_ratio = 360000u64;

    let mut shit = default_init(
        vec![
            PossibleShit::native_denom("uatom", atom_shit_ratio.clone().into()),
            PossibleShit::native_cw20(DEFAULT_CW20, cw20_shit_ratio.clone().into()),
        ],
        222u128,
    );

    let shitstrap = shit.shitstrap.clone();
    shit.setup_default_funds(shitstrap.clone())?;

    // error with wrong native token
    let err = shit
        .app
        .execute_contract(
            Addr::unchecked(SHITTER2.to_string()),
            shitstrap.clone(),
            &crate::msg::ExecuteMsg::ShitStrap {
                shit: AssetUnchecked::from_native("usilk", first_deposit),
            },
            &vec![coin(first_deposit, "usilk")],
        )
        .unwrap_err();
    assert_eq!(ContractError::WrongShit {}, err.downcast().unwrap());

    // create another cw20
    let cw20_init = Cw20Init {
        name: "poo".into(),
        symbol: "POO".into(),
        decimals: 6,
        initial_balances: vec![
            Cw20Coin {
                address: SHITTER2.to_string(),
                amount: 1000u128.into(),
            },
            Cw20Coin {
                address: SHITTER3.to_string(),
                amount: 1000u128.into(),
            },
        ],
        mint: None,
        marketing: None,
    };

    let bad_cw20 = shit
        .app
        .instantiate_contract(
            1u64,
            Addr::unchecked(OWNER),
            &cw20_init,
            &[],
            "cw20",
            Some(OWNER.into()),
        )
        .unwrap();

    // error with wrong cw20
    let err = shit
        .app
        .execute_contract(
            bad_cw20,
            shitstrap.clone(),
            &crate::msg::ExecuteMsg::Receive(Cw20ReceiveMsg {
                sender: SHITTER2.to_string(),
                amount: 200u128.into(),
                msg: to_json_binary(&ReceiveMsg::ShitStrap {
                    shit_strapper: SHITTER2.to_string(),
                })?,
            }),
            &vec![],
        )
        .unwrap_err();
    assert_eq!(ContractError::WrongShit {}, err.downcast().unwrap());

    // user 1 funds with native
    shit.participate_native(SHITTER1, 100_000_000, "uatom")?;
    // confirm shit_rate is calculated correctly
    let res: Uint128 = shit
        .app
        .wrap()
        .query_wasm_smart(shitstrap.clone(), &crate::msg::QueryMsg::ShitPile {})?;
    assert_eq!(
        res,
        Uint128::new(first_deposit) * Decimal::from_atomics(atom_shit_ratio, ATOMINC_DECIMALS)?
    );
    // confirm funds made it to shitter
    let res = shit.app.wrap().query_all_balances(SHITTER1)?;
    assert_eq!(
        res,
        vec![
            coin(DEFAULT_BALANCE - first_deposit, "uatom"),
            coin(
                (Uint128::new(first_deposit)
                    * Decimal::from_atomics(atom_shit_ratio, ATOMINC_DECIMALS)?)
                .u128(),
                "ushit"
            )
        ]
    );

    // user 2 funds with coin. should reflect 50% shit weight of native
    shit.participate_cw20(SHITTER2, 100_000_000, "contract0")?;

    // confirm shit_rate is calculated correctly
    let res: Uint128 = shit
        .app
        .wrap()
        .query_wasm_smart(shitstrap.clone(), &crate::msg::QueryMsg::ShitPile {})?;

    // the expected shit_strapped, after 2 participants
    let expected = (Uint128::new(first_deposit)
        * Decimal::from_atomics(cw20_shit_ratio, ATOMINC_DECIMALS)?)
        + (Uint128::new(first_deposit) * Decimal::from_atomics(atom_shit_ratio, ATOMINC_DECIMALS)?);

    assert_eq!(res, expected);

    // confirm native token balance is correct
    let res = shit.app.wrap().query_all_balances(SHITTER2)?;
    assert_eq!(
        res,
        vec![
            coin(
                (Uint128::new(first_deposit)
                    * Decimal::from_atomics(cw20_shit_ratio, ATOMINC_DECIMALS)?)
                .u128(),
                "ushit"
            ),
            coin(DEFAULT_BALANCE, "usilk"), // has full balance of non accepted token
        ]
    );
    // we skip checking cw20 balance in this test, as we simulate the contract callling headstash.
    Ok(())
}

// test shit strap w/ cw20, not using cw20 recieve
#[test]
fn test_cw20_receive() -> anyhow::Result<(), Error> {
    let mut shit = default_init(
        vec![PossibleShit::native_cw20(DEFAULT_CW20, 100u128)],
        222u128,
    );

    let first_deposit = 100u128;

    let shitstrap = shit.shitstrap.clone();
    shit.setup_default_funds(shitstrap.clone())?;

    // cannot directly call shit_strap entry point with cw20
    let err = shit
        .app
        .execute_contract(
            Addr::unchecked(SHITTER1.to_string()),
            shitstrap.clone(),
            &crate::msg::ExecuteMsg::ShitStrap {
                shit: AssetUnchecked {
                    denom: cw_denom::UncheckedDenom::Cw20(DEFAULT_CW20.into()),
                    amount: first_deposit.into(),
                },
            },
            &vec![],
        )
        .unwrap_err();

    assert_eq!(ContractError::ShittyCw20 {}, err.downcast().unwrap());

    // only cw20 can call receive entry point
    let err = shit
        .app
        .execute_contract(
            Addr::unchecked(SHITTER2.to_string()),
            shitstrap.clone(),
            &crate::msg::ExecuteMsg::Receive(Cw20ReceiveMsg {
                sender: SHITTER2.to_string(),
                amount: first_deposit.into(),
                msg: to_json_binary(&ReceiveMsg::ShitStrap {
                    shit_strapper: SHITTER2.to_string(),
                })?,
            }),
            &vec![],
        )
        .unwrap_err();

    assert_eq!(ContractError::WrongShit {}, err.downcast().unwrap());
    Ok(())
}
