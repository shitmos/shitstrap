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

    #[error("you are trying to set identical accepted_shit values. Try again with unique accepted_shit")]
    SameShit {},

    #[error("You are trying to set too much shit to be accepted for this shitstrap.")]
    UnnaceptableShitAmount {},

    #[error("Cannot set cutoff as 0")]
    ShittyCutoffRatio {},
    
    #[error("Cannot set conversion ratio as 0")]
    ShittyConversionRatio {},
    
    #[error("Shitstrap title set for too long")]
    ShittyTitle {},
    
    #[error("Shitstrap title set for too long")]
    ShittyDescription {},

    #[error("You are trying to participate with a cw20. use the Cw20RecieveMsg.")]
    ShittyCw20{},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
