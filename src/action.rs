use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::source::download_torrent;
use crate::source::xml::structs::{self, Channel, Torrent};

use crate::target::api::client::Client;
use crate::target::api::contexts::torrent::responses::UploadedTorrent;
use crate::target::api::contexts::user::responses::LoggedInUserData;
use crate::target::request::{login, upload_torrent, Error};

/// It logs in a user in the target Index API.
pub async fn log_in_target_api(config: &Config) -> LoggedInUserData {
    let unauthenticated_client = Client::unauthenticated(&config.target.url);

    println!(
        "API login with username {} ...",
        config.target.username.to_string().green()
    );

    let user: LoggedInUserData = login(
        &unauthenticated_client,
        &config.target.username,
        &config.target.password,
    )
    .await;

    if user.admin {
        println!("Logged as admin with account {} ", user.username.green());
    } else {
        println!("Logged as {} ", user.username.green());
    }

    user
}

/// It parses the source XML file and extracts the ``channel`` attribute.
///
/// # Panics
///
/// It panics if the source file cannot be read or if the XML file cannot be parsed.
#[must_use]
pub fn parse_xml_source(config: &Config) -> Channel {
    println!("Reading file {} ...", config.source.filename.green());

    let content =
        fs::read_to_string(config.source.filename.clone()).expect("Unable to read XML file.");

    let rss: structs::Rss =
        serde_xml_rs::from_str(&content).expect("Unable to parse XML source file.");

    rss.channel
}

/// It downloads the torrent file from the source URL and uploads it to the
/// torrust Index suing the API.
///
/// # Errors
///
/// It returns an error if the torrent file cannot be uploaded to the
/// Torrust Index.
///
/// # Panics
///
/// It panics if the torrent file cannot be downloaded or if the torrent cannot
/// be uploaded to the Torrust Index.
pub async fn process_torrent(
    config: &Config,
    target_api_client: &Client,
    torrent: &Torrent,
) -> Result<UploadedTorrent, Error> {
    let torrent_path = download_torrent_from_source(config, torrent).await;

    upload_torrent_to_target(target_api_client, torrent, &torrent_path).await
}

async fn download_torrent_from_source(config: &Config, torrent: &Torrent) -> PathBuf {
    println!("Downloading the torrent file from the source server ...");

    let torrent_url = format!(
        "{}{}.torrent",
        config.source.torrent_source_url, torrent.infohash
    );

    let torrent_filename = format!("{}.torrent", torrent.infohash);
    let torrent_path = Path::new(&config.source.torrents_dir).join(torrent_filename);

    if torrent_path.exists() {
        println!(
            "Torrent file cached in {}",
            torrent_path.display().to_string().green()
        );
    } else {
        println!(
            "Downloading torrent from {torrent_url} into {}",
            torrent_path.display().to_string().green()
        );

        download_torrent(&torrent_url, &torrent_path)
            .await
            .expect("msg: Couldn't download the torrent.");

        println!("Torrent successfully downloaded.");
    }

    torrent_path
}

async fn upload_torrent_to_target(
    target_api_client: &Client,
    torrent: &Torrent,
    torrent_path: &Path,
) -> Result<UploadedTorrent, Error> {
    println!(
        "Uploading the torrent {} to the Torrust Index ...",
        &torrent.infohash.green()
    );

    match upload_torrent(target_api_client, torrent, torrent_path).await {
        Ok(uploaded_torrent) => Ok(uploaded_torrent),
        Err(error) => Err(error),
    }
}
