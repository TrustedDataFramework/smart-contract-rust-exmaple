use wbi::U256;
use rlp_derive::{RlpDecodable, RlpEncodable};

#[derive(RlpDecodable, RlpEncodable)]
pub struct PoolData {
    pool_type: u8,
    funding: U256,
    reserve: U256,
    debt: U256,
    price: U256,
    decimals: u8    
}