use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Map, Item};

use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub nfts_in_chain : u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NFT {
    pub listed : bool,
    pub owner : Addr,
    pub name : String,
    pub description : String,
    pub price : u128,
}

pub const STATE: Item<State> = Item::new("state");
pub const NFTLIST: Map<String, NFT> = Map::new("nfts");