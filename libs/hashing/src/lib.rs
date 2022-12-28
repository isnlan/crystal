use core::hash::Hasher;

use byteorder::{ByteOrder, LittleEndian};
use digest::Digest;

/// Do a XX 128-bit hash and place result in `dest`.
pub fn twox_128_into(data: &[u8], dest: &mut [u8; 16]) {
    let r0 = twox_hash::XxHash::with_seed(0).chain_update(data).finish();
    let r1 = twox_hash::XxHash::with_seed(1).chain_update(data).finish();
    LittleEndian::write_u64(&mut dest[0..8], r0);
    LittleEndian::write_u64(&mut dest[8..16], r1);
}

/// Do a XX 128-bit hash and return result.
pub fn twox_128(data: &[u8]) -> [u8; 16] {
    let mut r: [u8; 16] = [0; 16];
    twox_128_into(data, &mut r);
    r
}

/// Do a keccak 256-bit hash and return result.
pub fn keccak_256(data: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    output.copy_from_slice(sha3::Keccak256::digest(data).as_slice());
    output
}
