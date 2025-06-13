use crate::model::Transfer;
use anyhow::Result;
use clickhouse::Client;

pub struct ClickhouseStorage {
    client: Client,
}

impl ClickhouseStorage {
    pub fn new(database_url: &str) -> Self {
        let client = Client::default()
            .with_url(database_url)
            .with_user("default")
            .with_password("111");

        Self { client }
    }

    pub async fn insert_transfer(
        &self,
        transfer: &Transfer,
    ) -> Result<(), clickhouse::error::Error> {
        let mut insert = self.client.insert("transfers")?;
        insert.write(transfer).await?;
        insert.end().await?;
        Ok(())
    }

    pub async fn get_transfers(&self) -> Result<Vec<Transfer>, clickhouse::error::Error> {
        let transfers = self
            .client
            .query("SELECT * FROM transfers")
            .fetch_all::<Transfer>()
            .await?;
        Ok(transfers)
    }
}
