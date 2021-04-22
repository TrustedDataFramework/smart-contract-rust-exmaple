use wbi::U256;
use crate::utils::RayMath;
use wbi::log;
const _pi: &'static str = "314159265358979323846264338327950288419716939937510";


lazy_static! {
    pub(crate) static ref Pi: U256 = {
        let u: U256 = _pi[..28].parse().unwrap();
        RayMath::ray_div(
            &u, &U256::from(10).pow(27)
        )
    };  

    pub(crate) static ref PiDiv2: U256 = {
        (&*Pi) / U256::from(2)
    };      

    pub(crate) static ref PiDiv12: U256 = {
        (&*Pi) / U256::from(12)
    };    
    
    pub(crate) static ref PiDiv4: U256 = {
        (&*Pi) / U256::from(4)
    };   
    
    pub(crate) static ref _0_8: U256 = {
         RayMath::ray_div(&U256::from(8), &U256::from(10))
    };

    pub(crate) static ref _0_2: U256 = {
        RayMath::ray_div(&U256::from(2), &U256::from(10))
    };   
    
    pub(crate) static ref _0_4: U256 = {
        RayMath::ray_div(&U256::from(4), &U256::from(10))
    };        
}
