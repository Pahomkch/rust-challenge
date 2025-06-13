mod generator;
mod model;
mod stats;
mod storage;
use generator::{DefaultTransferGenerator, TransferGenerator};
use stats::calculate_user_stats;

#[tokio::main]
async fn main() {
    let storage = storage::ClickhouseStorage::new("http://localhost:8123");
    let mut transfers = storage.get_transfers().await.unwrap();

    if transfers.len() == 0 {
        let mock_transfers = DefaultTransferGenerator::default().generate(10_000);

        for transfer in mock_transfers.iter() {
            storage.insert_transfer(transfer).await.unwrap();
        }

        transfers = storage.get_transfers().await.unwrap();
    }

    let mut stats = calculate_user_stats(&transfers);
    stats.sort_by(|a, b| a.address.cmp(&b.address));

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
    }
}
