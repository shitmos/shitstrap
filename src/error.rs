use cosmwasm_std::StdError;
use cw_denom::DenomError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    DenomError(#[from] DenomError),
    
    #[error("Wrong Shit.")]
    WrongShit{},
    
    #[error("Full of Shit.")]
    FullOfShit{},

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
