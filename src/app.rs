use colored::Colorize;

use crate::action::{log_in_target_api, parse_xml_source, process_torrent};
use crate::config::Config;
use crate::source::xml::structs;
use crate::statistics::Statistics;
use crate::target::api::client::Client;
use crate::ui::print_channel_header;
use crate::ui::{print_statistics, print_torrent_header};

pub async fn run() {
    let config = Config::default();

    let mut statistics = Statistics::default();

    let api_user = log_in_target_api(&config).await;

    let target_api_client = Client::authenticated(&config.target.url, &api_user.token);

    let channel: structs::Channel = parse_xml_source(&config);

    print_channel_header(&channel);

    for torrent in &channel.item {
        statistics.processed_torrents_counter += 1;

        if statistics.processed_torrents_counter < config.start_processing_at_index {
            continue;
        }

        println!(
            "Processing torrent #{} ...",
            statistics.processed_torrents_counter.to_string().green()
        );

        if config.skipped_torrents_list.contains(&torrent.infohash) {
            println!(
                "Skipping torrent. Torrent {} is included on the skipped torrent list.",
                torrent.infohash
            );
            continue;
        }

        print_torrent_header(torrent);

        let response = target_api_client.get_torrent(&torrent.infohash).await;

        if response.is_ok() {
            println!(
                "Skipping torrent. Torrent {} already exists in the Torrust Index.",
                torrent.infohash.to_string().green()
            );
            continue;
        }

        match process_torrent(&config, &target_api_client, torrent).await {
            Ok(uploaded_torrent) => {
                statistics.uploaded_torrents_counter += 1;
                println!("Torrent uploaded {uploaded_torrent:#?}");
            }
            Err(error) => {
                println!(
                    "Error uploading torrent {}: {}",
                    torrent.infohash.green(),
                    error.to_string().red()
                );
            }
        }
    }

    print_statistics(&statistics);
}
