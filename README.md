# Torrents Importer Sample

[![Testing](https://github.com/torrust/torrents-importer-sample/actions/workflows/testing.yaml/badge.svg)](https://github.com/torrust/torrents-importer-sample/actions/workflows/testing.yaml)

This is an example application to import torrents from a source to the [Torrust Index](https://github.com/torrust/torrust-index-backend).

> NOTICE: It's not intended to be used in production, it's just an example.

If you already have a torrent index and you want to migrate to the Torrust Solution you can use this script as a base for building your custom importer.

## Usage

For the source of torrents, you need to rewrite that part depending on where you get your torrents from.

For the target, you need to change the target configuration to match your Index API.

```rust
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
```

## Known Issues

Some torrents might produce an error message. For more information see <https://github.com/torrust/torrust-index-backend/issues/266>.
