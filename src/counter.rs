use anyhow::Result;
use async_graphql::{Request, Response};
use linera_base::identifiers::ChainId;
use linera_sdk::base::{ContractAbi, ServiceAbi};
use linera_service::cli_wrappers::local_net::ProcessInbox;
use linera_service::cli_wrappers::ClientWrapper;
use log::info;

pub const COUNTER_SERVICE_PATH: &str = "src/assets/counter/counter_service.wasm";
pub const COUNTER_CONTRACT_PATH: &str = "src/assets/counter/counter_contract.wasm";

pub struct CounterAbi;

impl ContractAbi for CounterAbi {
    type Operation = u64;
    type Response = u64;
}

impl ServiceAbi for CounterAbi {
    type Query = Request;
    type QueryResponse = Response;
}

//// Runs a simple counter test case against a specified network by using
//// the provided client and node service.
pub async fn run_counter_case(
    chain: &ChainId,
    client: &ClientWrapper,
    node_service_port: u16,
) -> Result<()> {
    let original_counter_value = 35;
    let increment = 5;

    let current_dir = std::env::current_dir()?;
    let contract = current_dir.join(COUNTER_CONTRACT_PATH);
    let service = current_dir.join(COUNTER_SERVICE_PATH);

    let bytecode_id = client
        .publish_bytecode::<CounterAbi, (), u64>(contract, service, None)
        .await?;
    let application_id = client
        .create_application(&bytecode_id, &(), &original_counter_value, &[], None)
        .await?;

    let mut node_service = client
        .run_node_service(node_service_port, ProcessInbox::Skip)
        .await?;

    let application = node_service
        .make_application(&chain, &application_id)
        .await?;

    let counter_value: u64 = application.query_json("value").await?;
    assert_eq!(counter_value, original_counter_value);

    info!("Counter value is: {}", counter_value);

    let mutation = format!("increment(value: {increment})");
    application.mutate(mutation).await?;

    let counter_value: u64 = application.query_json("value").await?;
    assert_eq!(counter_value, original_counter_value + increment);

    info!("Counter value is: {}", counter_value);

    node_service.ensure_is_running()?;

    Ok(())
}
