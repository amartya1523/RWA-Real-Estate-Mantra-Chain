#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Addr, Order};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use cosmwasm_std::{BankMsg, Coin};
use cw2::set_contract_version;
use crate::state::{NFTLIST, State, STATE, NFT};
use crate::msg::QueryResponse;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};


const CONTRACT_NAME: &str = "crates.io:realesta";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State{
        nfts_in_chain : 0,
    };

    STATE.save(deps.storage, &state)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        MintNFT {should_list, nft_id, name, description, price, owner} => execute::mint_nft(deps, info, should_list, nft_id, name, description, price, owner),
        TransferNFT {sender, reciever, nft_id, should_list} => execute::transfer_nft(deps, info, should_list, sender, reciever, nft_id),
        SaleNFT {seller, buyer, nft_id} => execute::sale_nft(deps, info, seller, buyer, nft_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        NftInfo { nft_id} => to_binary(&query::info_query(deps, nft_id)?), 
        NftNum {} => to_binary(&query::num_nft(deps)?),
        NftListed {} => to_binary(&query::nft_listed(deps)?),
    }
}

mod execute {
    use super::*;

    pub fn mint_nft(deps: DepsMut, info: MessageInfo, should_list : bool, nft_id : String, nft_name : String, nft_description : String, nft_price : u128, nft_owner : Option<String>) -> Result<Response, ContractError>{
        let key = nft_id.clone();
        let value = NFTLIST.has(deps.storage, key);
        if value {
            return Err(ContractError::IDAlreadyinUse {  });
        }
    
        let owner = nft_owner.unwrap_or(info.sender.to_string());
        let own_addr : Addr = deps.api.addr_validate(&owner)?;
        let new_nft = NFT {
            listed : should_list,
            owner : own_addr,
            name: nft_name,
            description: nft_description,
            price : nft_price,
        };
    

        NFTLIST.save(deps.storage, nft_id, &new_nft)?;

        Ok(Response::default().add_attribute("action", "add").add_attribute("added", "NFT"))
    }
    
    pub fn transfer_nft(deps : DepsMut, info : MessageInfo, should_list : bool, sender : String, reciever : String, nft_id : String) -> Result<Response, ContractError> {
        let send_addr : Addr = deps.api.addr_validate(&sender)?;
        let recv_addr : Addr = deps.api.addr_validate(&reciever)?;

        if send_addr != info.sender {
            return Err(ContractError::NotOwner {  });
        }

        let key = nft_id.clone();
        let data = match NFTLIST.may_load(deps.storage, key)?{
            Some(data) => Some(data),
            None => return Err(ContractError::NftIDNotFound { id: nft_id }),
        }
        .unwrap();

        let updated_nft = NFT {
            listed : should_list,
            owner : recv_addr.clone(),
            name: data.name.clone(),
            description: data.description.clone(),
            price: data.price.clone(),
        };

        let nft_num = STATE.load(deps.storage).unwrap();
        let new_state = State {
            nfts_in_chain : nft_num.nfts_in_chain + 1,
        };

        STATE.save(deps.storage, &new_state)?;

        NFTLIST.save(deps.storage, nft_id, &updated_nft)?;

        Ok(Response::new().add_attribute("action", "Minted NFT"))
    }

    pub fn sale_nft(deps : DepsMut, info : MessageInfo, seller : String, buyer : String, nft_id : String) -> Result<Response, ContractError>{
        let key = nft_id.clone();
        let data = match NFTLIST.may_load(deps.storage, key)? {
            Some(data) => Some(data),
            None => return Err(ContractError::NftIDNotFound { id: nft_id }),
        }
        .unwrap();

        if data.listed {
        let buyer_balance = deps.querier.query_balance(info.sender.clone(), "uom".to_string())?;
        if buyer_balance.amount < data.price.into() {
            return Err(ContractError::InsufficientFunds{});
        }

        let coins = vec![Coin::new(data.price.into(), "uom".to_string())];

        let _send_msg = BankMsg::Send { to_address: seller.clone(), amount: coins};

        transfer_nft(deps, info, false, seller, buyer, nft_id)
    }
    else{
        return Err(ContractError::NFTNotListed {  });
    }

    }
}

mod query {
    use cosmwasm_std::StdError;
    use super::*;

    pub fn info_query(deps: Deps, nft_id: String) -> StdResult<QueryResponse> {
        let data = match NFTLIST.may_load(deps.storage, nft_id)? {
            Some(data) => Some(data),
            None => return Err(StdError::generic_err("NFT not found")),
        }
        .unwrap();

        let resp = QueryResponse{
            listed: data.listed.clone(),
            owner : data.owner.to_string().clone(),
            name : data.name.to_string().clone(),
            description: data.description.to_string().clone(),
            price: data.price.clone(),
        };

        Ok(resp)
    }
    
    pub fn num_nft(deps: Deps) -> StdResult<u128> {
        let num = STATE.load(deps.storage).unwrap();

        Ok(num.nfts_in_chain)
    }

    pub fn nft_listed(deps: Deps) -> StdResult<Vec<NFT>> {
        let mut to_list : Vec<NFT> = Vec::new();
        let keys: Vec<String> = NFTLIST.keys(deps.storage, None, None, Order::Ascending).collect::<Result<_, _>>().unwrap();

        for key in keys {
            let data = NFTLIST.load(deps.storage, key).unwrap();

            if data.listed {
                to_list.push(data);
            }
        }
        
        Ok(to_list)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use crate::contract::instantiate;
    use crate::msg::{ExecuteMsg, InstantiateMsg};

    use super::execute;

    pub const ADDR1 : &str = "addr1";
    pub const ADDR2 : &str = "addr2";

    #[test]
    fn test_mint(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = ExecuteMsg::MintNFT {should_list: true, nft_id: "this_sample".to_string(), name: "Sample".to_string(), description: "Sample test".to_string(), price: 30, owner : None };

        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }

    #[test]
    fn test_transfer(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg {};

        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg);

        let msg = ExecuteMsg::MintNFT {should_list: true, nft_id: "this_sample".to_string(), name: "Sample".to_string(), description: "Sample test".to_string(), price : 30, owner : None };

        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::TransferNFT {should_list: false, sender: ADDR1.to_string(), reciever: ADDR2.to_string(), nft_id: "this_sample".to_string()};

        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }

    #[test]
    fn test_sale(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg {};

        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg);

        let msg = ExecuteMsg::MintNFT {should_list: true, nft_id: "this_sample".to_string(), name: "Sample".to_string(), description: "Sample test".to_string(), price : 30, owner : None };

        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::SaleNFT { seller: ADDR1.to_string(), buyer: ADDR2.to_string(), nft_id: "this_sample".to_string() };

        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }
}
