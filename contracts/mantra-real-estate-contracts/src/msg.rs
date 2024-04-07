use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    MintNFT {should_list : bool, nft_id : String, name : String, description : String, price : u128, owner : Option<String>},
    TransferNFT {sender : String, reciever : String, nft_id : String, should_list : bool},
    SaleNFT {seller : String, buyer : String, nft_id : String},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryResponse {
    pub listed : bool,
    pub owner : String,
    pub name : String,
    pub description : String,
    pub price: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    NftInfo { nft_id : String },
    NftNum {},
    NftListed {},
}