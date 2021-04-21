use wbi::U256;
use alloc::string::*;

pub struct RayMath;

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
        &RAY
    }

    pub fn half_ray() -> &'static U256 {
        &HALF_RAY
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
        let halfB: U256 = b / U256::from(2);
    
        assert!(*a <= (U256::max() - &halfB) / RayMath::ray(), "multiply overflow");
    
        return (a * RayMath::ray() + &halfB) / b;
    }       
}

const chars: &'static str = "0123456789";

pub fn decimal(v: &U256, decimals: u32) -> String{
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


pub fn ray_pow(x: &U256, y: u32) -> U256 {
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
// export function sinxAndAdd(x: U256, add: U256, inverse: bool): U256 {
//     let power = x
//     let power_3 = rayPow2(x, 3)
//     let power_5 = RayMath.rayMul(RayMath.rayMul(power_3, x), x)
//     let power_7 = RayMath.rayMul(RayMath.rayMul(power_5, x), x)
//     let power_9 = RayMath.rayMul(RayMath.rayMul(power_7, x), x)
//     let power_11 = RayMath.rayMul(RayMath.rayMul(power_9, x), x)
//     let power_13 = RayMath.rayMul(RayMath.rayMul(power_11, x), x)
//     let power_15 = RayMath.rayMul(RayMath.rayMul(power_13, x), x)
//     let power_17 = RayMath.rayMul(RayMath.rayMul(power_15, x), x)
//     let power_19 = RayMath.rayMul(RayMath.rayMul(power_17, x), x)

//     let p1 = x 
//     let p2 = power_3 / U256.fromU64(fac(3))
//     let p3 = power_5 / U256.fromU64(fac(5))
//     let p4 = power_7 / U256.fromU64(fac(7))
//     let p5 = power_9 / U256.fromU64(fac(9))
//     let p6 = power_11 / U256.fromU64(fac(11))
//     let p7 = power_13 / U256.fromU64(fac(13))
//     let p8 = power_15 / U256.fromU64(fac(15))
//     let p9 = power_17 / U256.fromU64(fac(17))
//     let p10 = power_19 / U256.fromU64(fac(19))

//     if(!inverse) {
//         let ret = add + p1 + p3  + p5 + p7 + p9
//         // - p2 - p4 - p6
//         if( ret < p2 )
//             return U256.ZERO
//         ret = ret - p2
//         if(ret < p4)
//             return U256.ZERO
//         ret = ret - p4
//         if(ret < p6)
//             return U256.ZERO
//         ret = ret - p6
//         if(ret < p8)
//             return U256.ZERO
//         ret = ret - p8
//         if(ret < p10)
//             return U256.ZERO
//         return ret - p10
//     }
//     let ret = add + p2 + p4  + p6 + p8 + p10
//     // - p2 - p4 - p6
//     if( ret < p1 )
//         return U256.ZERO
//     ret = ret - p1
//     if(ret < p3)
//         return U256.ZERO
//     ret = ret - p3
//     if(ret < p5)
//         return U256.ZERO
//     ret = ret - p5
//     if(ret < p7)
//         return U256.ZERO
//     ret = ret - p7
//     if(ret < p9)
//         return U256.ZERO
//     return ret - p9
// }