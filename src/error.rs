use cosmwasm_std::{DecimalRangeExceeded, StdError};
use cw_denom::DenomError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    ShitStd(#[from] StdError),

    #[error("{0}")]
    ShitDenomError(#[from] DenomError),
    
    #[error("{0}")]
    DecimalRangeExceeded(#[from] DecimalRangeExceeded),

    #[error("Wrong Shit.")]
    WrongShit {},

    #[error("Full of Shit.")]
    FullOfShit {},

    #[error("Did Not Send Shit, Liar.")]
    DidntSendShit {},
    
    #[error("Unable to claim refund")]
    DigginForShitTreasure {},
    
    #[error("Unauthorized")]
    ShittyAuthorization {},

    #[error("You are trying to participate with a cw20. use the Cw20RecieveMsg.")]
    ShittyCw20{},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
