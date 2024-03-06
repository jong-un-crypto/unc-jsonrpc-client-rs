use unc_jsonrpc_client::methods;
use unc_jsonrpc_primitives::types::query::QueryResponseKind;
use unc_primitives::types::{AccountId, BlockReference, Finality};
use unc_primitives::views::QueryRequest;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = utils::select_network()?;

    let account_id: AccountId = utils::input("Enter an Account ID to lookup: ")?.parse()?;

    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::ViewAccount { account_id },
    };

    let response = client.call(request).await?;

    if let QueryResponseKind::ViewAccount(result) = response.kind {
        println!("{:#?}", result);
    }

    Ok(())
}
