use anyhow::Result;

use linera_base::identifiers::ChainId;
use linera_service::cli_wrappers::ClientWrapper;
use log::info;

/// Path to the small blob file
pub const SMALL_BLOB_PATH: &str = "src/assets/blob/500kb_file.bin";
/// Path to the medium blob file
pub const MEDIUM_BLOB_PATH: &str = "src/assets/blob/2mb_file.bin";
/// Path to the large blob file
pub const LARGE_BLOB_PATH: &str = "src/assets/blob/7mb_file.bin";

/// Runs a simple blob test case against a specified network by using
/// the provided client and node service.
///
/// The test case publishes three different blobs with sizes of 500 kb, 2 mb, and 7 mb.
pub async fn run_blob_case(chain: &ChainId, client: &ClientWrapper) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let small_blob = current_dir.join(SMALL_BLOB_PATH);
    let medium_blob = current_dir.join(MEDIUM_BLOB_PATH);
    let large_blob = current_dir.join(LARGE_BLOB_PATH);
    // let blob = current_dir.join(BLOB_PATH);

    let small_blob_hash = client.publish_data_blob(&small_blob, Some(*chain)).await?;

    info!(
        "Published small blob (500 kb) with hash: {}",
        small_blob_hash
    );

    let medium_blob_hash = client.publish_data_blob(&medium_blob, Some(*chain)).await?;

    info!(
        "Published medium blob (2 mb) with hash: {}",
        medium_blob_hash
    );

    let large_blob_hash = client.publish_data_blob(&large_blob, Some(*chain)).await?;

    info!("Published large blob (7 mb) with hash: {}", large_blob_hash);

    Ok(())
}
