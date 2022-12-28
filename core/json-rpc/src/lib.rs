mod server;
mod signer;

pub use rpc_core::EthApiServer;
pub use server::Server;
pub use signer::*;

pub fn internal_err<T: ToString>(message: T) -> jsonrpsee::core::Error {
    err(jsonrpsee::types::error::INTERNAL_ERROR_CODE, message, None)
}

pub fn err<T: ToString>(code: i32, message: T, data: Option<&[u8]>) -> jsonrpsee::core::Error {
    jsonrpsee::core::Error::Call(jsonrpsee::types::error::CallError::Custom(
        jsonrpsee::types::error::ErrorObject::owned(
            code,
            message.to_string(),
            data.map(|bytes| {
                jsonrpsee::core::to_json_raw_value(&format!("0x{}", hex::encode(bytes)))
                    .expect("fail to serialize data")
            }),
        ),
    ))
}
