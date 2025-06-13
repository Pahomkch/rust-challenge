mod generator;
mod model;
mod stats;
mod storage;
use anyhow::{Context, Result};
use generator::{DefaultTransferGenerator, TransferGenerator};
use stats::calculate_user_stats;

#[tokio::main]
async fn main() -> Result<()> {
    let storage = storage::ClickhouseStorage::new("http://localhost:8123");
    let mut transfers = storage
        .get_transfers()
        .await
        .context("Failed to get transfers from storage")?;

    if transfers.len() == 0 {
        let mock_transfers = DefaultTransferGenerator::default()
            .generate(10_000)
            .context("Failed to generate mock transfers")?;

        for transfer in mock_transfers.iter() {
            storage
                .insert_transfer(transfer)
                .await
                .context("Failed to insert transfer into storage")?;
        }

        transfers = storage
            .get_transfers()
            .await
            .context("Failed to get transfers from storage after inserting mock transfers")?;
    }

    let mut stats = calculate_user_stats(&transfers).context("Failed to calculate user stats")?;
    stats.sort_by(|a, b| a.address.cmp(&b.address));

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
    }

    Ok(())
}
