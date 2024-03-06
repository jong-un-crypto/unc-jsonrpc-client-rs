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
        utils::input("Enter the contract whose code we're downloading: ")?.parse()?;

    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::ViewCode {
            account_id: contract_id.clone(),
        },
    };

    let response = client.call(request).await?;

    if let QueryResponseKind::ViewCode(result) = response.kind {
        let path = format!("/tmp/{}.wasm", contract_id);
        println!("⚙️  [{}]", contract_id);
        println!("🏋        size: {} bytes", result.code.len());
        std::fs::write(&path, result.code)?;
        println!("💾   saved to: {}", path);
    }

    Ok(())
}
