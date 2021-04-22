use wbi::U256;
use alloc::string::*;
use crate::{constants::{PiDiv12, _0_8, PiDiv4, _0_2, PiDiv2, Pi, _0_4}, types::pool_type};
use crate::types::{self, PoolData, PoolInfo, MineParams};
use alloc::vec::Vec;

pub(crate) struct RayMath;

lazy_static! {
    static ref RAY: U256 = {
        U256::from(10u64).pow(27)
    };

    static ref HALF_RAY: U256 = {
        U256::from(10u64).pow(26) * U256::from(5)
    };    
}

impl RayMath {
    pub fn ray() -> &'static U256 {
        &*RAY
    }

    pub fn half_ray() -> &'static U256 {
        &*HALF_RAY
    }
    pub fn ray_mul(a: &U256, b: &U256) -> U256{
        if a.is_zero() || b.is_zero() {
            return U256::zero();
        }

        let n = (U256::max() - RayMath::half_ray()) / b;
        assert!(a <= &n, "raymath: multiply overflow");    
        return (a * b + RayMath::half_ray()) / RayMath::ray();
    }   

    pub fn ray_div(a: &U256, b: &U256) -> U256{
        assert!(!b.is_zero(), "raymath: divided by zero");
        let half_b: U256 = b / U256::from(2);
    
        assert!(*a <= (U256::max() - &half_b) / RayMath::ray(), "multiply overflow");
    
        return (a * RayMath::ray() + &half_b) / b;
    }       
}

const chars: &'static str = "0123456789";

pub(crate) fn decimal(v: &U256, decimals: u32) -> String{
    if v.is_zero() {
        return "0".to_string();
    }
    let mut i = 1;
    let mut ret = String::new();

    let base = U256::from(10);

    let mut all_zero = true;
    let mut v = v.clone();

    while !v.is_zero() || i <= decimals {
        let remainder = &v % &base;
        v = v / &base;

        let ch = remainder.u64() as usize;

        all_zero = all_zero && (ch == 0);

        if all_zero && i <= decimals {
        } else {
            ret.insert(0, chars.as_bytes()[ch] as char);
        }
        if i == decimals {
            if !all_zero {
                ret.insert(0, '.');
            }
            if v.is_zero() {
                ret.insert(0, '0')
            }
        }
        i += 1;
    }
    ret
}


pub(crate) fn ray_pow(x: &U256, y: u32) -> U256 {
    let mut z = RayMath::ray().clone();
    for _ in 0..y {
        z = RayMath::ray_mul(&z, x)
    }
    z
}

pub fn fac(u: u64) -> u64 {
    let mut r: u64 = 1;

    for i in 1..=u {
        r = r * i        
    }
    r
}



// // 泰勒展开式 sinx = x - x^3/3! + x^5/5! - x^7/7! + x^9/9! - x^11/11! + x^13/13! - x^15/15! + x^17/17! - x^19/19!
pub(crate) fn sinx_and_add(x: &U256, add: &U256, inverse: bool) -> U256 {
    let power_3 = ray_pow(x, 3);

    let power_5 = RayMath::ray_mul(&RayMath::ray_mul(&power_3, x), x);
    let power_7 = RayMath::ray_mul(&RayMath::ray_mul(&power_5, x), x);
    let power_9 = RayMath::ray_mul(&RayMath::ray_mul(&power_7, x), x);
    let power_11 = RayMath::ray_mul(&RayMath::ray_mul(&power_9, x), x);
    let power_13 = RayMath::ray_mul(&RayMath::ray_mul(&power_11, x), x);
    let power_15 = RayMath::ray_mul(&RayMath::ray_mul(&power_13, x), x);
    let power_17 = RayMath::ray_mul(&RayMath::ray_mul(&power_15, x), x);
    let power_19 = RayMath::ray_mul(&RayMath::ray_mul(&power_17, x), x);

    let p1 = x.clone();
    let p2 = power_3 / U256::from(fac(3));
    let p3 = power_5 / U256::from(fac(5));
    let p4 = power_7 / U256::from(fac(7));
    let p5 = power_9 / U256::from(fac(9));
    let p6 = power_11 / U256::from(fac(11));
    let p7 = power_13 / U256::from(fac(13));
    let p8 = power_15 / U256::from(fac(15));
    let p9 = power_17 / U256::from(fac(17));
    let p10 = power_19 / U256::from(fac(19));

    if !inverse {
        let mut ret = add + p1 + p3  + p5 + p7 + p9;
        if ret < p2 {
            return U256::zero();
        }
        ret = ret - p2;

        if ret < p4 {
            return U256::zero();
        }
        ret = ret - p4;

        if ret < p6 {
            return U256::zero();
        }
        ret = ret - p6;
        if ret < p8 {
            return U256::zero();            
        }
        ret = ret - p8;
        if ret < p10 {
            return U256::zero();
        }
        return ret - p10;
    }
    let mut ret = add + p2 + p4  + p6 + p8 + p10;
    if ret < p1 {
        return U256::zero();
    }
    ret = ret - p1;

    if ret < p3 {
        return U256::zero();
    }
    ret = ret - p3;

    if ret < p5 {
        return U256::zero();
    }
    ret = ret - p5;
    if ret < p7 {
        return U256::zero();
    }
    ret = ret - p7;
    if ret < p9 {
        return U256::zero();
    }
    return ret - p9;
}

pub fn calculate_svrb(v_maze: &U256, v_usp: &U256) -> U256{
    if v_maze.is_zero() {
        return U256::zero();
    }

    if v_usp.is_zero() {
        return U256::zero();
    }

    let p_maze = RayMath::ray_div(v_maze, v_usp);

    if &p_maze < &*PiDiv12 {
        return _0_8.clone();
    }
   
    if &p_maze > &*PiDiv4 {
        return _0_2.clone();
    }
    
    let _5_3 = RayMath::ray_div(
        &U256::from(5),
        &U256::from(3)
    );

    let mut p_maze_6 = p_maze * U256::from(6);

    if &p_maze_6 >= &*PiDiv2 && &p_maze_6 < &*Pi {
        p_maze_6 = &*Pi - p_maze_6; // sin(x) = - sin(x - pi) = sin(pi - x)
    }

    let inverse = &p_maze_6 >= &*Pi; // sin(x + pi) = - sin(x)

    let x = if inverse { p_maze_6 - &*Pi }  else { p_maze_6 };

    let y = sinx_and_add(
        &x,
        &_5_3,
        inverse
    );
    
    return y * U256::from(3) / U256::from(10);
}

// TODO: calculate forb maze by maze occupy, expressed in ray
pub fn calculate_forb(debt: &U256, funding: &U256) -> U256{
    if funding.is_zero() {
        return U256::zero();
    }

    let occupy = RayMath::ray_div(debt, funding);

    if &occupy <= &*_0_2 {
        return RayMath::half_ray().clone();
    }

    if &occupy <= &*_0_8 {
        return occupy / U256::from(2) + &*_0_4;
    }
    return _0_8.clone();
}

pub(crate) fn calculate_share(_pools: &[PoolData], _params: &MineParams) -> Vec<PoolInfo> {
    let y = RayMath::ray_mul(&_params.y_global, &_params.k);
    let y_usp = RayMath::ray_mul(&y, &_params.r_usp);
    let y_isp = RayMath::ray_mul(&y, &_params.r_isp);
    let y_dp: U256 = &y - &y_usp - &y_isp;

    let mut ret: Vec<PoolInfo> = Vec::with_capacity(_pools.len());
    let mut v_usp = U256::zero();
    let mut v_isp = U256::zero();
    let mut v_stable = U256::zero();
    let mut v_maze = U256::zero();
    let mut svrb = U256::zero();
    let mut dp_index: usize = 0;
    

    for i in 0.._pools.len() {
        let p = &_pools[i];
        let info: PoolInfo = p.into();
        
        if info.total.is_zero() {
            ret.push(info);
            continue;
        }

        if p.pool_type == types::pool_type::USP  {
            v_usp = v_usp + &info.total_value;

            if i == 0 {
                v_maze = v_maze + &info.total_value;
            }else{
                v_stable = v_stable + &info.total_value;
            }
        }

        if p.pool_type == types::pool_type::ISP {
            v_isp = v_isp + &info.total_value;
        }


        if p.pool_type == types::pool_type::DP {
            dp_index = i;
        }        

        ret.push(info)        

    }

    // update forb, share for maze
    svrb = calculate_svrb(&v_maze, &v_usp);
    let maze_info = &mut ret[0];
    maze_info.update_share(&y_usp, svrb.clone(), calculate_forb(&_pools[0].debt, &maze_info.funding));
    let d_maze = RayMath::ray_mul(&y_usp, &svrb);
    let d_stable = &y_usp - &d_maze;

    for i in 1.._pools.len() {
        let info = &mut ret[i];

        if info.pool_type == pool_type::USP && !v_stable.is_zero(){
            info.update_share(
                &d_stable, 
                RayMath::ray_div(&info.total_value, &v_stable),
                calculate_forb(&_pools[i].debt, &info.funding)
            )
        } 

        if info.pool_type == pool_type::ISP && !v_isp.is_zero(){
            info.update_share(
                &y_isp, 
                RayMath::ray_div(&info.total_value, &v_isp),
                calculate_forb(&_pools[i].debt, &info.funding)
            )
        }         

        if  info.pool_type == pool_type::DP && !info.funding.is_zero() {
            info.funding_share = RayMath::ray_div(
                &y_dp,
                &info.funding
            )
        }

    }
    return ret;
}