use cosmwasm_std::{coin, Addr, Decimal, Empty, Fraction, Uint128};
use cw20::Cw20Coin;
use cw20_base::msg::InstantiateMsg as Cw20Init;
use cw_multi_test::{App, AppResponse, BankSudo, Contract, ContractWrapper, Executor, SudoMsg};
use cw_orch::anyhow::{self, Error};

use crate::{
    msg::{AssetUnchecked, InstantiateMsg, PossibleShit},
    ContractError,
};

pub const OWNER: &str = "owner";
pub const SHITTER1: &str = "shitter1";
pub const SHITTER2: &str = "shitter2";

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

fn default_init(possible: Vec<PossibleShit>) -> ShitSuite {
    // create simulation environment
    let mut app = App::default();
    let cw20_id = app.store_code(cw20_contract());
    let shitstrap_id = app.store_code(shitstrap_contract());

    let cw20_init = Cw20Init {
        name: "poo".into(),
        symbol: "POO".into(),
        decimals: 6,
        initial_balances: vec![Cw20Coin {
            address: SHITTER2.to_string(),
            amount: 1000u128.into(),
        }],
        mint: None,
        marketing: None,
    };

    let init = InstantiateMsg {
        owner: OWNER.into(),
        accepted: possible,
        cutoff: 222u128.into(),
        shitmos: "ushit".into(),
    };

    // instantiate contract with cw20
    let shitstrap = instantiate_w_cw20(app, shitstrap_id, init, cw20_id, cw20_init);
    shitstrap
}

pub struct ShitSuite {
    pub app: App,
    pub shitstrap: Addr,
    pub cw20: Addr,
}

impl ShitSuite {
    fn setup_default_funds(&mut self, shitstrap: Addr) -> cw_orch::anyhow::Result<(), Error> {
        self.app
            .sudo(SudoMsg::Bank(BankSudo::Mint {
                to_address: OWNER.into(),
                amount: vec![coin(1000u128, "uatom")],
            }))
            .unwrap();
        self.app
            .sudo(SudoMsg::Bank(BankSudo::Mint {
                to_address: SHITTER1.into(),
                amount: vec![coin(1_000u128, "uatom")],
            }))
            .unwrap();
        self.app
            .sudo(SudoMsg::Bank(BankSudo::Mint {
                to_address: SHITTER2.into(),
                amount: vec![coin(1_000u128, "usilk")],
            }))
            .unwrap();
        // fund shitstrap with 1 million shit
        self.app
            .sudo(SudoMsg::Bank(BankSudo::Mint {
                to_address: shitstrap.to_string(),
                amount: vec![coin(1_000_000, "ushit")],
            }))
            .unwrap();
        Ok(())
    }
    fn participate_cw20(&mut self, sender: &str, amount: u128) -> Result<AppResponse, Error> {
        self.app.execute_contract(
            Addr::unchecked(sender.to_string()),
            self.shitstrap.clone(),
            &crate::msg::ExecuteMsg::ShitStrap {
                shit: AssetUnchecked {
                    denom: cw_denom::UncheckedDenom::Cw20("poo".into()),
                    amount: amount.into(),
                },
            },
            &vec![],
        )
    }
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
    let mut shit = default_init(vec![PossibleShit {
        token: cw_denom::UncheckedDenom::Native("uatom".into()),
        shit_rate: Decimal::one(),
    }]);

    let first_deposit = 221u128;

    let shitstrap = shit.shitstrap.clone();
    shit.setup_default_funds(shitstrap.clone())?;

    // try to participate in shitstrap with wrong native token
    let err = shit
        .app
        .execute_contract(
            Addr::unchecked(OWNER.to_string()),
            shitstrap.clone(),
            &crate::msg::ExecuteMsg::ShitStrap {
                shit: AssetUnchecked {
                    denom: cw_denom::UncheckedDenom::Native("usilk".into()),
                    amount: first_deposit.into(),
                },
            },
            &vec![coin(first_deposit, "uatom")],
        )
        .unwrap_err();
    assert_eq!(ContractError::WrongShit {}, err.downcast().unwrap());
    // try to participate in shitstrap with wrong cw20 token
    let err = shit.participate_cw20(OWNER, first_deposit).unwrap_err();

    assert_eq!(ContractError::WrongShit {}, err.downcast().unwrap());

    // try to participate without sending token
    let err = shit
        .app
        .execute_contract(
            Addr::unchecked(SHITTER1.to_string()),
            shitstrap.clone(),
            &crate::msg::ExecuteMsg::ShitStrap {
                shit: AssetUnchecked {
                    denom: cw_denom::UncheckedDenom::Native("uatom".into()),
                    amount: first_deposit.into(),
                },
            },
            &vec![],
        )
        .unwrap_err();

    assert_eq!(ContractError::DidntSendShit {}, err.downcast().unwrap());

    // participate in shitstrap with correct token, but less sent then specified
    let mut block = shit.app.block_info();
    block.height += 1;
    shit.app.set_block(block);

    shit.app
        .execute_contract(
            Addr::unchecked(SHITTER1.to_string()),
            shitstrap.clone(),
            &crate::msg::ExecuteMsg::ShitStrap {
                shit: AssetUnchecked {
                    denom: cw_denom::UncheckedDenom::Native("uatom".into()),
                    amount: first_deposit.into(),
                },
            },
            &vec![coin(22, "uatom")],
        )
        .unwrap_err();
    // participate in shitstrap with correct token
    let mut block = shit.app.block_info();
    block.height += 1;
    shit.app.set_block(block);

    shit.participate_native(SHITTER1, 221, "uatom")?;
    // confirm shit_rate is calculated correctly
    let res: Uint128 = shit
        .app
        .wrap()
        .query_wasm_smart(shitstrap.clone(), &crate::msg::QueryMsg::ShitPile {})?;
    assert_eq!(res, Uint128::new(first_deposit));

    // confirm new balance of shitstrap
    let balance = shit.app.wrap().query_balance(shitstrap.clone(), "uatom")?;
    let shit_rate: Option<Decimal> = shit.app.wrap().query_wasm_smart(
        shitstrap.clone(),
        &crate::msg::QueryMsg::ShitRate {
            asset: "uatom".to_string(),
        },
    )?;
    let calculated = balance.amount * shit_rate.unwrap();
    assert_eq!(calculated, Uint128::from(first_deposit));

    // shitstrap reaches limit
    shit.app.execute_contract(
        Addr::unchecked(SHITTER1.to_string()),
        shitstrap.clone(),
        &crate::msg::ExecuteMsg::ShitStrap {
            shit: AssetUnchecked {
                denom: cw_denom::UncheckedDenom::Native("uatom".into()),
                amount: 2u128.into(),
            },
        },
        &vec![coin(2u128, "uatom")],
    )?;

    let mut block = shit.app.block_info();
    block.height += 1;
    shit.app.set_block(block);

    // confirm contract will not continue to shitstrap
    let res: bool = shit
        .app
        .wrap()
        .query_wasm_smart(shit.shitstrap, &crate::msg::QueryMsg::FullOfShit {})?;
    assert_eq!(res, true);

    let balance = shit.app.wrap().query_balance(SHITTER1, "uatom")?;
    assert_eq!(balance.amount, Uint128::from(779u128));

    // no more shitstrapping can commence
    let err = shit
        .app
        .execute_contract(
            Addr::unchecked(SHITTER1.to_string()),
            shitstrap.clone(),
            &crate::msg::ExecuteMsg::ShitStrap {
                shit: AssetUnchecked {
                    denom: cw_denom::UncheckedDenom::Native("uatom".into()),
                    amount: 2u128.into(),
                },
            },
            &vec![coin(2u128, "uatom")],
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

    let mut block = shit.app.block_info();
    block.height += 1;
    shit.app.set_block(block);

    let balance = shit.app.wrap().query_balance(SHITTER1, "uatom")?;
    assert_eq!(balance.amount, Uint128::from(780u128));

    Ok(())
}
