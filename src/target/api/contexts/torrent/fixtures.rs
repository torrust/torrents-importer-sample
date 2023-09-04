use super::forms::{BinaryFile, UploadTorrentMultipartForm};

use super::responses::Id;

/// Information about a torrent that is going to added to the index.
#[derive(Clone)]
pub struct TorrentIndexInfo {
    pub title: String,
    pub description: String,
    pub category: String,
    pub torrent_file: BinaryFile,
}

impl From<TorrentIndexInfo> for UploadTorrentMultipartForm {
    fn from(indexed_torrent: TorrentIndexInfo) -> UploadTorrentMultipartForm {
        UploadTorrentMultipartForm {
            title: indexed_torrent.title,
            description: indexed_torrent.description,
            category: indexed_torrent.category,
            torrent_file: indexed_torrent.torrent_file,
        }
    }
}

/// Torrent that has been added to the index.
pub struct TorrentListedInIndex {
    pub torrent_id: Id,
    pub title: String,
    pub description: String,
    pub category: String,
    pub torrent_file: BinaryFile,
}

impl TorrentListedInIndex {
    #[must_use]
    pub fn from(torrent_to_index: TorrentIndexInfo, torrent_id: Id) -> Self {
        Self {
            torrent_id,
            title: torrent_to_index.title,
            description: torrent_to_index.description,
            category: torrent_to_index.category,
            torrent_file: torrent_to_index.torrent_file,
        }
    }
}
