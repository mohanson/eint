use arbitrary::{Arbitrary, Unstructured};
use eint::E256;
use std::{
    convert::{From, TryInto},
    intrinsics::transmute,
};
use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

impl From<E256> for U256 {
    fn from(e: E256) -> Self {
        let arr = e2562array(e);
        U256::from_little_endian(&arr)
    }
}

impl From<U256> for E256 {
    fn from(u: U256) -> Self {
        let mut arr = [0u8; 32];
        u.to_little_endian(&mut arr);
        array2e256(arr)
    }
}

pub fn array2e256(data: [u8; 32]) -> E256 {
    unsafe { transmute::<[u8; 32], E256>(data) }
}

pub fn e2562array(e: E256) -> [u8; 32] {
    unsafe { transmute::<E256, [u8; 32]>(e) }
}

pub fn gen_uint256_pair(data: &[u8]) -> Uint256Pair {
    let a = U256::from_little_endian(&data[0..32]);
    let b = U256::from_little_endian(&data[32..64]);
    Uint256Pair { a, b }
}

pub fn gen_eint256_pair(data: &[u8]) -> Eint256Pair {
    let data_a: [u8; 32] = data[0..32].try_into().unwrap();
    let data_b: [u8; 32] = data[32..64].try_into().unwrap();

    let a: E256 = array2e256(data_a);
    let b: E256 = array2e256(data_b);

    Eint256Pair { a, b }
}

pub struct Uint256Pair {
    pub a: U256,
    pub b: U256,
}

impl<'a> Arbitrary<'a> for Uint256Pair {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let array = u.arbitrary::<[u8; 64]>()?;
        Ok(gen_uint256_pair(&array))
    }
}

pub struct Eint256Pair {
    pub a: E256,
    pub b: E256,
}

impl<'a> Arbitrary<'a> for Eint256Pair {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let array = u.arbitrary::<[u8; 64]>()?;
        Ok(gen_eint256_pair(&array))
    }
}
