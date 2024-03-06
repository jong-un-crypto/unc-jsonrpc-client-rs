//! Queries the current state of node network connections.
//!
//! This includes information about active peers, transmitted data, known producers, etc.
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
//! let request = methods::network_info::RpcNetworkInfoRequest;
//!
//! let response = client.call(request).await;
//!
//! assert!(matches!(
//!     response,
//!     Ok(methods::network_info::RpcNetworkInfoResponse { .. })
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use unc_jsonrpc_primitives::types::network_info::{
    RpcNetworkInfoError, RpcNetworkInfoResponse,
};

#[derive(Debug)]
pub struct RpcNetworkInfoRequest;

impl RpcHandlerResponse for RpcNetworkInfoResponse {}

impl RpcHandlerError for RpcNetworkInfoError {}

impl RpcMethod for RpcNetworkInfoRequest {
    type Response = RpcNetworkInfoResponse;
    type Error = RpcNetworkInfoError;

    fn method_name(&self) -> &str {
        "network_info"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(null))
    }
}

impl private::Sealed for RpcNetworkInfoRequest {}
