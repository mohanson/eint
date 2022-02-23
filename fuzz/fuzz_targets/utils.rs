use eint::E256;
use std::{
    convert::{From, TryInto},
    intrinsics::transmute,
};
use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

pub fn array2e256(data: [u8; 32]) -> E256 {
    unsafe { transmute::<[u8; 32], E256>(data) }
}

pub fn e2562array(e: E256) -> [u8; 32] {
    unsafe { transmute::<E256, [u8; 32]>(e) }
}

pub fn gen_uint256_pair(data: &[u8]) -> (U256, U256) {
    let a = U256::from_little_endian(&data[0..32]);
    let b = U256::from_little_endian(&data[32..64]);
    (a, b)
}

pub fn gen_eint256_pair(data: &[u8]) -> (E256, E256) {
    let data_a: [u8; 32] = data[0..32].try_into().unwrap();
    let data_b: [u8; 32] = data[32..64].try_into().unwrap();

    let a: E256 = array2e256(data_a);
    let b: E256 = array2e256(data_b);

    (a, b)
}
