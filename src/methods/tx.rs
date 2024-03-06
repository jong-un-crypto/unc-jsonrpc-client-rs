//! Queries the status of a transaction.
//!
//! ## Example
//! Returns the final transaction result for
//! <https://explorer.unc.org/transactions/B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq>
//!
//! ```no_run
//! use unc_jsonrpc_client::{methods, JsonRpcClient};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.unc.org");
//! let tx_hash = "B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq".parse()?;
//!
//! let request = methods::tx::RpcTransactionStatusRequest {
//!     transaction_info: methods::tx::TransactionInfo::TransactionId {
//!         tx_hash,
//!         sender_account_id: "itranscend.unc".parse()?,
//!    }
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert_eq!(tx_hash, response.transaction.hash);
//! # Ok(())
//! # }
//! ```
use super::*;

pub use unc_jsonrpc_primitives::types::transactions::RpcTransactionError;
pub use unc_jsonrpc_primitives::types::transactions::TransactionInfo;

pub type RpcTransactionStatusResponse = unc_primitives::views::FinalExecutionOutcomeView;

#[derive(Debug)]
pub struct RpcTransactionStatusRequest {
    pub transaction_info: TransactionInfo,
}

impl From<RpcTransactionStatusRequest>
    for unc_jsonrpc_primitives::types::transactions::RpcTransactionStatusRequest
{
    fn from(this: RpcTransactionStatusRequest) -> Self {
        Self {
            transaction_info: this.transaction_info,
            wait_until: unc_primitives::views::TxExecutionStatus::None,
        }
    }
}

impl RpcMethod for RpcTransactionStatusRequest {
    type Response = RpcTransactionStatusResponse;
    type Error = RpcTransactionError;

    fn method_name(&self) -> &str {
        "tx"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(match &self.transaction_info {
            TransactionInfo::Transaction(signed_transaction) => {
                match signed_transaction {
                    unc_jsonrpc_primitives::types::transactions::SignedTransaction::SignedTransaction(tx) => {
                        json!([common::serialize_signed_transaction(tx)?])
                    }
                }
            }
            TransactionInfo::TransactionId { tx_hash,sender_account_id, ..} => {
                json!([tx_hash, sender_account_id])
            }
        })
    }
}

impl private::Sealed for RpcTransactionStatusRequest {}
