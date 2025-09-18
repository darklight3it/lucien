*“So this is your library, huh, Lucien? It's a big place What's so special about it, then?"*

*"Oh, it's a very unusual library, Matthew. Somewhere in here is every story that has every been dreamed. "*

*"They're just books."*

*"Oh yes. But unusual books. You'll find none of them on Earth. In this section, for example, are novels their authors never wrote, or never finished, except in dreams.”*
— Neil Gaiman, The Sandman, Vol. 4: Season of Mists

# Lucien

Lucien is a command-line application written in Rust to help you organize and enrich your ebook library. It fetches metadata for your ebooks from authoritative online sources like the OpenLibrary API and helps you keep your collection consistent and well-organized.

## Features

*   **Metadata Fetching:** Automatically fetches ebook metadata from the OpenLibrary API.
*   **EPUB Support:** Reads and writes metadata to EPUB files.
*   **Configurable:** Easily configurable through a `Config.toml` file.

## Usage

To use Lucien, you can run it from the command line, providing the path to an ebook file:

```bash
cargo run -- --file /path/to/your/book.epub
```

## Configuration

Lucien uses a `Config.toml` file for configuration. Here's an example:

```toml
# The log level for the application.
# Possible values are: "trace", "debug", "info", "warn", "error"
log_level = "info"
```

## Future Features

*   Hooks on a folder for automatic ebook processing.