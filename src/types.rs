use wbi::U256;
use rlp_derive::{RlpDecodable, RlpEncodable};
use crate::utils::RayMath;

pub mod pool_type {
    pub const USP: u64 = 0;
    pub const ISP: u64 = 1;
    pub const DP: u64 = 2;
}

#[derive(RlpDecodable, RlpEncodable, Default)]
pub struct PoolData {
    pub pool_type: u64,
    pub funding: U256,
    pub reserve: U256,
    pub debt: U256,
    pub price: U256,
    pub decimals: u64    
}

#[derive(RlpDecodable, RlpEncodable)]
pub struct MineParams {
    pub y_global: U256, 
    pub k: U256, 
    pub r_usp: U256, 
    pub r_dp: U256,
    pub r_isp: U256
}

#[derive(RlpDecodable, RlpEncodable, Default)]
pub struct PoolInfo {
    pub pool_type: u64, 
    pub funding: U256,
    pub reserve: U256,
    pub total: U256,
    pub funding_value: U256,
    pub reserve_value: U256,
    pub total_value: U256,
    pub forb: U256,
    pub vwd: U256,
    pub funding_share: U256,
    pub reserve_share: U256    
}

impl From<&PoolData> for PoolInfo {
    fn from(p: &PoolData) -> PoolInfo {
        let mut info = PoolInfo::default();
        info.pool_type = p.pool_type;
        info.funding = p.funding.clone();
        info.reserve = p.reserve.clone();
        info.total = &info.funding + &info.reserve;
        let decimal_divisor = U256::from(10).pow(p.decimals);
        info.funding_value = &info.funding * &p.price / &decimal_divisor;
        info.reserve_value = &info.reserve * &p.price / &decimal_divisor;
        info.total_value = &info.funding_value + &info.reserve_value; 
        info       
    }
}

impl PoolInfo {
    pub fn update_share(&mut self, y: &U256, vwd: U256, forb: U256) {   
        self.vwd = vwd;
        self.forb = forb;

        let reward = RayMath::ray_mul(y, &self.vwd);
    
        let funding_reward = RayMath::ray_mul(&reward, &self.forb);
        let reserve_reward = &reward - &funding_reward;

        if !self.funding.is_zero() {
            self.funding_share = RayMath::ray_div(&funding_reward, &self.funding);
        }
    
        if !self.reserve.is_zero() {
            self.reserve_share = RayMath::ray_div(&reserve_reward, &self.reserve);
        }
    
    }
}