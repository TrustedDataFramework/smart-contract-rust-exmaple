use wbi::{Address, db::{Store, Globals}};
use wbi::U256;
use alloc::string::*;

fn get_chain_id() -> u64{
    Globals::get("_chain_id").unwrap_or(0)
}

fn set_chain_id(id: u64){
    Globals::insert("_chain_id", &id);
}

fn get_owner() -> Address{
    Globals::get("_owner").unwrap_or_default()
}

fn set_owner(owner: &Address){
    Globals::insert("_owner", owner);
}