use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("NFT ID already in use")]
    IDAlreadyinUse {},

    #[error("Invalid wallet ID: {id:?}")]
    InvalidID { id : String },

    #[error("NFT ID not found: {id:?}")]
    NftIDNotFound { id : String },

    #[error("Sender is not Owner")]
    NotOwner {},

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("NFT not listed")]
    NFTNotListed {},
}
