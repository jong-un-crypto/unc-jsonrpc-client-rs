use unc_jsonrpc_client::methods;
use unc_jsonrpc_primitives::types::query::QueryResponseKind;
use unc_primitives::types::{AccountId, BlockReference, Finality};
use unc_primitives::views::QueryRequest;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = utils::select_network()?;

    let contract_id: AccountId =
        utils::input("Enter the contract whose state you want to inspect: ")?.parse()?;

    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::ViewState {
            account_id: contract_id.clone(),
            prefix: unc_primitives::types::StoreKey::from(Vec::new()),
            include_proof: false,
        },
    };

    let response = client.call(request).await?;

    if let QueryResponseKind::ViewState(result) = response.kind {
        println!("{:#?}", result);
    }

    Ok(())
}
