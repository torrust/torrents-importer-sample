pub mod responses;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Settings {
    pub website: Website,
    pub tracker: Tracker,
    pub net: Network,
    pub auth: Auth,
    pub database: Database,
    pub mail: Mail,
    pub image_cache: ImageCache,
    pub api: Api,
    pub tracker_statistics_importer: TrackerStatisticsImporter,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Website {
    pub name: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Tracker {
    pub url: String,
    pub mode: String,
    pub api_url: String,
    pub token: String,
    pub token_valid_seconds: u64,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Network {
    pub port: u16,
    pub base_url: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Auth {
    pub email_on_signup: String,
    pub min_password_length: usize,
    pub max_password_length: usize,
    pub secret_key: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Database {
    pub connect_url: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Mail {
    pub email_verification_enabled: bool,
    pub from: String,
    pub reply_to: String,
    pub username: String,
    pub password: String,
    pub server: String,
    pub port: u16,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ImageCache {
    pub max_request_timeout_ms: u64,
    pub capacity: usize,
    pub entry_size_limit: usize,
    pub user_quota_period_seconds: u64,
    pub user_quota_bytes: usize,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Api {
    pub default_torrent_page_size: u8,
    pub max_torrent_page_size: u8,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct TrackerStatisticsImporter {
    pub torrent_info_update_interval: u64,
}
