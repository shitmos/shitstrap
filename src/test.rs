use cosmwasm_std::{coin, Addr, Empty, Uint128};
use cw20_base::msg::InstantiateMsg as Cw20Init;
use cw_multi_test::{App, BankSudo, Contract, ContractWrapper, Executor, SudoMsg};

use crate::msg::{InstantiateMsg, PossibleShit};

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
    let mut app = App::default();
    let cw20_id = app.store_code(cw20_contract());
    let shitstrap_id = app.store_code(shitstrap_contract());

    let cw20_init = Cw20Init {
        name: "poo".into(),
        symbol: "POO".into(),
        decimals: 6,
        initial_balances: vec![],
        mint: None,
        marketing: None,
    };

    let init = InstantiateMsg {
        owner: OWNER.into(),
        accepted: possible,
        cutoff: 2222u128.into(),
        shitmos: "ushit".into(),
    };

    let shitstrap = instantiate_w_cw20(app, shitstrap_id, init, cw20_id, cw20_init);
    shitstrap
}

fn setup_default_funds(mut app: App, shitstrap: Addr) {
    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: OWNER.into(),
        amount: vec![coin(1_000u128, "uatom")],
    }))
    .unwrap();
    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: SHITTER1.into(),
        amount: vec![coin(1_000u128, "uatom")],
    }))
    .unwrap();
    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: SHITTER2.into(),
        amount: vec![coin(1_000u128, "usilk")],
    }))
    .unwrap();
    // fund shitstrap with 1 million shit
    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: shitstrap.to_string(),
        amount: vec![coin(1_000_000, "ushit")],
    }))
    .unwrap();
}

pub struct ShitSuite {
    pub app: App,
    pub shitstrap: Addr,
    pub cw20: Addr,
}
#[test]
fn test_shitstrap() {
    let shit = default_init(vec![PossibleShit {
        token: cw_denom::UncheckedDenom::Native("uatom".into()),
        shit_rate: Uint128::new(100u128),
    }]);

    setup_default_funds(shit.app, shit.shitstrap);

    // try to participate in shitstrap with wrong token

    // participate in shitstrap with correct token

    // shitstrap reaches limit, returns excess funds

    // no more shitstrapping can commence
}
