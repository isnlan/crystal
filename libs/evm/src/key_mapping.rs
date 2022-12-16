use codec::Encode;
use hashing::twox_128;
use storage::storage_prefix;

pub struct StorageDoubleMap {
    pub module: Vec<u8>,
    pub storage: Vec<u8>,
}

impl StorageDoubleMap {
    pub fn storage_double_map_final_key<KArg1, KArg2>(&self, k1: KArg1, k2: KArg2) -> Vec<u8>
    where
        KArg1: Encode,
        KArg2: Encode,
    {
        let storage_prefix = storage_prefix(&self.module, &self.storage);
        let key1_hashed = k1.using_encoded(twox_128);
        let key2_hashed = k2.using_encoded(twox_128);

        let mut final_key = Vec::with_capacity(
            storage_prefix.len() + key1_hashed.as_ref().len() + key2_hashed.as_ref().len(),
        );

        final_key.extend_from_slice(&storage_prefix);
        final_key.extend_from_slice(key1_hashed.as_ref());
        final_key.extend_from_slice(key2_hashed.as_ref());

        final_key
    }

    pub fn storage_double_map_key_prefix<KArg1>(&self, k1: KArg1) -> Vec<u8>
    where
        KArg1: Encode,
    {
        let storage_prefix = storage_prefix(&self.module, &self.storage);
        let key1_hashed = k1.using_encoded(twox_128);

        let mut final_key = Vec::with_capacity(storage_prefix.len() + key1_hashed.as_ref().len());

        final_key.extend_from_slice(&storage_prefix);
        final_key.extend_from_slice(key1_hashed.as_ref());

        final_key
    }

    pub fn storage_double_map_prefix(&self) -> Vec<u8> {
        let storage_prefix = storage_prefix(&self.module, &self.storage);

        let mut final_key = Vec::with_capacity(storage_prefix.len());

        final_key.extend_from_slice(&storage_prefix);

        final_key
    }
}
