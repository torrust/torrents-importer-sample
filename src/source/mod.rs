pub mod xml;

use std::fs;
use std::path::Path;

/// It downloads a torrent file from the given URL and saves it in the given path
///
/// # Errors
///
/// It returns an error if the torrent file cannot be downloaded or the response
/// cannot be read.
///
/// # Panics
///
/// Panics if the torrent file cannot be written.
pub async fn download_torrent(url: &str, torrent_path: &Path) -> Result<(), reqwest::Error> {
    let response = reqwest::get(url).await?;

    assert!(
        response.status() == 200,
        "Could not download torrent file from {}. Response status: {}",
        url,
        response.status()
    );

    let bytes = response.bytes().await?;
    fs::write(torrent_path, bytes).expect("could not write torrent file");
    Ok(())
}
