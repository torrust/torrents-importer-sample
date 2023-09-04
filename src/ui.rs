use colored::Colorize;

use crate::{
    source::xml::structs::{Channel, Torrent},
    statistics::Statistics,
    utils::truncate_string,
};

/// It prints the channel attributes before starting processing it.
pub fn print_channel_header(channel: &Channel) {
    println!("Processing channel ...");
    println!("  Title: {}", channel.title.green());
    println!("  description: {}", channel.description.green());
    println!("  Link: {}", channel.link.green());

    if channel.item.len() == 1 {
        println!(
            "Processing only {} torrent ...",
            channel.item.len().to_string().green()
        );
    } else {
        println!(
            "Processing {} torrents ...",
            channel.item.len().to_string().green()
        );
    }
}

/// It prints the torrent attributes before starting processing it.
pub fn print_torrent_header(torrent: &Torrent) {
    println!("  Title: {}", torrent.title.green());
    println!("  Category: {}", torrent.category.green());
    println!("  Infohash: {}", torrent.infohash.green());
    println!("  Guid: {}", torrent.guid.green());
    println!("  Link: {}", torrent.link.green());
    println!(
        "  Description: {} ...",
        truncate_string(&torrent.description, 30).green()
    );
    println!("  Size: {}", torrent.size.to_string().green());

    println!(
        "Checking if the torrent {} exist in the Torrust Index ...",
        torrent.infohash.to_string().green()
    );
}

pub fn print_statistics(statistics: &Statistics) {
    println!(
        "Processed torrents: {}",
        statistics.processed_torrents_counter.to_string().green()
    );
    println!(
        "Uploaded torrents:  {}",
        statistics.uploaded_torrents_counter.to_string().green()
    );
}
