/// Returns the storage prefix for a specific pallet name and storage name.
///
/// The storage prefix is `concat(twox_128(pallet_name), twox_128(storage_name))`.
pub fn storage_prefix(module_name: &[u8], storage_name: &[u8]) -> [u8; 32] {
    let pallet_hash = hashing::twox_128(module_name);
    let storage_hash = hashing::twox_128(storage_name);

    let mut final_key = [0u8; 32];
    final_key[..16].copy_from_slice(&pallet_hash);
    final_key[16..].copy_from_slice(&storage_hash);

    final_key
}
