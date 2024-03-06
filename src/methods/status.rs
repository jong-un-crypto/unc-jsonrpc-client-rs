//! Requests the status of the connected RPC node.
//!
//! This includes information about sync status, unccore node version, protocol version, the current set of validators, etc.
//!
//! ## Example
//!
//! ```
//! use unc_jsonrpc_client::{methods, JsonRpcClient};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://rpc.testnet.unc.org");
//!
//! let request = methods::status::RpcStatusRequest;
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::status::RpcStatusResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use unc_jsonrpc_primitives::types::status::RpcStatusError;

pub type RpcStatusResponse = unc_primitives::views::StatusResponse;

#[derive(Debug)]
pub struct RpcStatusRequest;

impl RpcHandlerResponse for RpcStatusResponse {}

impl RpcMethod for RpcStatusRequest {
    type Response = RpcStatusResponse;
    type Error = RpcStatusError;

    fn method_name(&self) -> &str {
        "status"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(null))
    }
}

impl private::Sealed for RpcStatusRequest {}
