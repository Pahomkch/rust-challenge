mod common;
mod generator;
mod model;
mod stats;
mod storage;
use anyhow::{Context, Result};
use common::ClickhouseClient;
use generator::{DefaultTransferGenerator, TransferGenerator};
use stats::{calculate_user_stats_clickhouse, calculate_user_stats_rust};

#[tokio::main]
async fn main() -> Result<()> {
    let storage = storage::ClickhouseStorage::new("http://localhost:8123");

    let mut transfers = storage
        .get_transfers()
        .await
        .context("Failed to get transfers from storage.")?;

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

    let mut stats_clickhouse =
        calculate_user_stats_clickhouse(&ClickhouseClient::new("http://localhost:8123"))
            .await
            .context("Failed to calculate user stats")?;
    stats_clickhouse.sort_by(|a, b| a.address.cmp(&b.address));

    for stat in stats_clickhouse.iter().take(10) {
        println!("Clickhouse: \n{:?}", stat);
    }

    let mut stats_rust =
        calculate_user_stats_rust(&transfers).context("Failed to calculate user stats")?;
    stats_rust.sort_by(|a, b| a.address.cmp(&b.address));

    for stat in stats_rust.iter().take(10) {
        println!("{:?}", stat);
    }
    Ok(())
}
