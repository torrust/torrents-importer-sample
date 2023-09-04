/// Configures the importer.
pub struct Config {
    /// Where to get the torrents from.
    pub source: Source,
    /// Index API configuration to upload the torrents to.
    pub target: Target,
    /// Ignores the first torrents in the source until it reaches this index.
    pub start_processing_at_index: u64,
    /// List of torrents to exclude in the importation process.
    pub skipped_torrents_list: Vec<String>,
}

/// Where to get the torrents from.
pub struct Source {
    pub filename: String,
    pub torrents_dir: String,
    pub torrent_source_url: String,
}

/// Index API configuration to upload the torrents to.
pub struct Target {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            source: Source {
                filename: "./data/database.xml".to_string(),
                torrents_dir: "./data/torrents".to_string(),
                torrent_source_url: "https://raw.githubusercontent.com/torrust/torrents-importer-sample/main/tests/fixtures/torrents/".to_string(),
            },
            target: Target {
                url: "localhost:3001".to_string(),
                username: "admin".to_string(),
                password: "12345678".to_string(),
            },
            start_processing_at_index: 0,
            skipped_torrents_list: vec![],
        }
    }
}
