use wbi::{Address, db::{Store, Globals}};
use wbi::U256;
use wbi::context::msg;
use alloc::string::*;
use alloc::vec::Vec;

lazy_static! {
    static ref _harvest_ratio: Store<Address, U256> = Store::new("_harvest_ratio");
    static ref _pairs: Store<u64, Vec<u8>> = Store::new("_pairs");
    static ref _pair_index: Store<Vec<u8>, u64> = Store::new("_pair_index");
    static ref _unclaimed: Store<Vec<u8>, U256> = Store::new("_unclaimed");

}

macro_rules! concat_bytes {
    ($l: expr, $r: expr) => {
        {
            let mut v = Vec::with_capacity($l.len() + $r.len());
            v.extend_from_slice($l);
            v.extend_from_slice($r);
            v
        }
    };
}

pub(crate) fn get_unclaimed(pair: &[u8]) -> U256{
    _unclaimed.get(&pair.to_vec()).unwrap_or_default()
}

pub(crate) fn set_unclaimed(pair: &[u8], amount: U256) {
    _unclaimed.insert(&pair.to_vec(), &amount)
}


pub(crate) fn get_pair_by_index(i: u64) -> Vec<u8> {
    return _pairs.get(&i).unwrap()
}

pub(crate) fn get_chain_id() -> u64{
    Globals::get("_chain_id").unwrap_or(0)
}

pub(crate) fn set_chain_id(id: u64){
    Globals::insert("_chain_id", &id);
}

pub(crate) fn get_owner() -> Address{
    Globals::get("_owner").unwrap_or_default()
}

pub(crate) fn set_owner(owner: &Address){
    Globals::insert("_owner", owner);
}

pub(crate) fn require_owner() {
    assert!(&get_owner() == &msg.sender);
}

pub(crate) fn get_harvest_ratio_limit(asset: &Address) -> U256 {
    _harvest_ratio.get(asset).unwrap_or_default()
}

pub(crate) fn set_harvest_ratio_limit(asset: &Address, limit: &U256) {
    _harvest_ratio.insert(asset, limit)
}

pub(crate) fn derive_pair_key(asset: &Address, mptype: u64, user: &Address) -> Vec<u8> {
    let mut r = concat_bytes!(asset.as_slice(), &mptype.to_be_bytes());
    concat_bytes!(&r, user.as_slice())
}

pub(crate) fn pairs_count() -> u64{
    Globals::get("_pairs_count").unwrap_or(0)
}

pub(crate) fn set_pairs_count(n: u64) {
    Globals::insert("_pairs_count", &n);
}

// save pair and get pair id
pub(crate) fn save_pair(asset: &Address, mptype: u64, user: &Address) -> u64{
    let k = derive_pair_key(asset, mptype, user);

    match _pair_index.get(&k) {
        Some(n) => n, 
        _ => {
            let i = pairs_count() + 1;
            _pair_index.insert(&k, &i);
            _pairs.insert(&i, &k);
            i
        }
    }
}