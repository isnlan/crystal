#[cfg(test)]
mod tests {
    use super::*;
    use crate::H256;
    use ethereum::Account;
    use std::str::FromStr;

    #[test]
    fn it_works() {
        let acc = Account {
            balance: 8u8.into(),
            nonce: 2u8.into(),
            storage_root: H256::from_str(
                "a3db671bd0653a641fb031dccb869982da390eade9e6f993802ed09c4f6b7b2a",
            )
            .unwrap(),
            code_hash: H256::from_str(
                "71623f5ec821de33ad5aa81f8c82f0916c6f60de0a536f8c466d440c56715bd5",
            )
            .unwrap(),
        };
        let data = rlp::encode(&acc);

        let acc1: Account = rlp::decode(data.as_ref()).unwrap();

        assert_eq!(acc, acc1)
    }
}
