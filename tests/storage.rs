use anyhow::Result;
use rust_challenge::domain::Transfer;
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
struct MockClient {
    pub inserted: Arc<Mutex<Vec<Transfer>>>,
}

impl MockClient {
    pub fn new() -> Self {
        Self {
            inserted: Arc::new(Mutex::new(vec![])),
        }
    }
    pub async fn insert_transfer(&self, transfer: &Transfer) -> Result<()> {
        self.inserted.lock().unwrap().push(transfer.clone());
        Ok(())
    }
    pub async fn get_transfers(&self) -> Result<Vec<Transfer>> {
        Ok(self.inserted.lock().unwrap().clone())
    }
}

struct TestStorage {
    client: MockClient,
}

impl TestStorage {
    pub fn new() -> Self {
        Self {
            client: MockClient::new(),
        }
    }
    pub async fn insert_transfer(&self, transfer: &Transfer) -> Result<()> {
        self.client.insert_transfer(transfer).await
    }
    pub async fn get_transfers(&self) -> Result<Vec<Transfer>> {
        self.client.get_transfers().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_transfer() -> Transfer {
        Transfer {
            ts: 1,
            address_from: "A".to_string(),
            address_to: "B".to_string(),
            amount: 100.0,
            usd_price: 1.5,
        }
    }

    #[tokio::test]
    async fn test_insert_and_get_transfer() {
        let storage = TestStorage::new();
        let transfer = sample_transfer();
        storage.insert_transfer(&transfer).await.unwrap();
        let transfers = storage.get_transfers().await.unwrap();
        assert_eq!(transfers.len(), 1);
        assert_eq!(transfers[0].amount, 100.0);
        assert_eq!(transfers[0].address_from, "A");
        assert_eq!(transfers[0].address_to, "B");
        assert_eq!(transfers[0].usd_price, 1.5);
    }

    #[tokio::test]
    async fn test_multiple_inserts() {
        let storage = TestStorage::new();
        let t1 = sample_transfer();
        let mut t2 = t1.clone();
        t2.amount = 200.0;
        storage.insert_transfer(&t1).await.unwrap();
        storage.insert_transfer(&t2).await.unwrap();
        let transfers = storage.get_transfers().await.unwrap();
        assert_eq!(transfers.len(), 2);
        assert_eq!(transfers[1].amount, 200.0);
    }
}
