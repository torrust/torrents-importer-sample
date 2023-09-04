#[derive(Debug, Deserialize)]
pub struct Rss {
    pub channel: Channel,
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub title: String,
    pub description: String,
    pub link: String,
    pub item: Vec<Torrent>,
}

#[derive(Debug, Deserialize)]
pub struct Torrent {
    pub title: String,
    pub category: String,
    pub infohash: String,
    pub guid: String,
    pub link: String,
    pub description: String,
    pub size: u64,
}
