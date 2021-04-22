#![no_std]
#![feature(default_alloc_error_handler)]

extern crate wee_alloc;

#[macro_use]
extern crate wbi;

use_wee_alloc!();  

use alloc::{vec::Vec};
use alloc::string::*;
extern crate hex;
extern crate rlp_derive;

#[macro_use]
extern crate lazy_static;

extern "C" {
    pub fn __log(a: u64); 
}

fn log_u64(x: u64) {
    unsafe {
        __log(x);
    }
}

mod constants;
mod persist;
mod types;
mod utils;

use hex::ToHex;
use lazy_static::lazy_static;
pub use wbi::{__change_t, __malloc, __peek, log, ret};
use wbi::db;
use wbi::{Address, U256 };
use wbi::context::{self, msg};
use utils::{RayMath, calculate_svrb, decimal, sinx_and_add};
use persist::{derive_pair_key, get_chain_id, get_harvest_ratio_limit, get_owner, get_unclaimed, require_owner, save_pair, set_chain_id, set_harvest_ratio_limit, set_owner, set_unclaimed};
use types::{PoolInfo, PoolData, MineParams};

#[no_mangle]
pub fn init(chain_id: u64) -> &'static Address{
    set_owner(&msg.sender);
    set_chain_id(chain_id);
    ret(context::this())
}

#[no_mangle]
pub fn getOwner() -> &'static Address { 
    ret(get_owner())
}

#[no_mangle]
pub fn getChainId() -> u64 { 
    get_chain_id()
}

#[no_mangle]
pub fn getHarvestRatioLimit(asset: Address) -> &'static U256 {
    ret(get_harvest_ratio_limit(&asset))
}

#[no_mangle]
pub fn setHarvestRatioLimit(asset: Address, limit: U256) {
    require_owner();
    set_harvest_ratio_limit(&asset, &limit);
}

#[no_mangle]
pub fn locked(asset: Address, mptype: u64, user: Address) -> &'static U256 {
    let pair = derive_pair_key(&asset, mptype, &user);
    let unclaimed = get_unclaimed(&pair);
    let ratio = get_harvest_ratio_limit(&asset);
    let unlocked = RayMath::ray_mul(&unclaimed, &ratio);
    ret(unclaimed - unlocked)
}

#[no_mangle]
pub fn unlocked(asset: Address, mptype: u64, user: Address) -> &'static U256 {
    let pair = derive_pair_key(&asset, mptype, &user);
    let unclaimed = get_unclaimed(&pair);
    let ratio = get_harvest_ratio_limit(&asset);
    ret(RayMath::ray_mul(&unclaimed, &ratio))
}

#[no_mangle]
pub fn modifyOwner(owner: Address) {
    require_owner();
    set_owner(&msg.sender);
}

#[no_mangle]
pub fn mint(asset: Address, mptype: u64, user: Address, amount: U256) {
    require_owner();
    let k =  derive_pair_key(&asset, mptype, &user);
    _mint(&k, &amount)
}

#[no_mangle]
pub fn batchMint(_assets: Vec<u8>, _mptypes: Vec<u8>, _users: Vec<u8>, _amounts: Vec<u8>, age: u64) {
    require_owner();
    let assets: Vec<Address> = rlp::decode_list(&_assets);
    let mptypes: Vec<u64> = rlp::decode_list(&_mptypes);
    let users: Vec<Address>  = rlp::decode_list(&_users);
    let amounts: Vec<U256> = rlp::decode_list(&_amounts);

    for i in 0.._assets.len() {
        let k = derive_pair_key(&assets[i], mptypes[i], &users[i]);
        save_pair(&assets[i], mptypes[i], &users[i]);
        _mint(&k, &amounts[i]);   
    }
}

#[no_mangle]
pub fn burn(asset: Address, mptype: u64, user: Address, amount: U256, age: u64) {
    require_owner();
    let k =  derive_pair_key(&asset, mptype, &user);
    _burn(&k, &amount)
}

#[no_mangle]
pub fn calculateSVRB(v_maze: U256, v_usp: U256) -> &'static U256{
    ret(calculate_svrb(&v_maze, &v_usp))
}

#[no_mangle]
pub fn calculateShare(_pools: Vec<u8>, _params: Vec<u8>) -> &'static Vec<u8> {
    let pools: Vec<PoolData>  = rlp::decode_list(&_pools);
    let params: MineParams = rlp::decode(&_params).unwrap();
    let r = utils::calculate_share(&pools, &params);
    let encoded = rlp::encode_list(&r);
    ret(encoded.to_vec())
}


fn _mint(pair: &[u8], amount: &U256) {
    let beforeMint = get_unclaimed(pair);
    let afterMint = beforeMint + amount;
    set_unclaimed(pair, afterMint);
}


fn _burn(pair: &[u8], amount: &U256){
    let beforeBurn = get_unclaimed(pair);
    let afterBurn = if &beforeBurn <= amount { U256::zero() }  else { beforeBurn - amount };
    set_unclaimed(pair, afterBurn)
}
