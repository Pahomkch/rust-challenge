use clickhouse::Client;

pub struct ClickhouseClient {
    pub client: Client,
}

impl ClickhouseClient {
    pub fn new(database_url: &str) -> Self {
        let client = Client::default()
            .with_url(database_url)
            .with_user("default")
            .with_password("111");
        Self { client }
    }
}
