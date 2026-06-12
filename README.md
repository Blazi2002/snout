# Snout

Search words, topics and content across all your files — locally, fast, and private.

Snout indexes the files on your machine and lets you find exactly which file
contains what you're looking for: a word, a phrase, a title, or a topic.
Everything runs locally. Your data never leaves your computer.

## Status

Early development. Snout currently performs plain-text search across the files
in a given folder.

## Usage

    snout <folder> <word>

Example:

    snout ./documents invoice

## Building from source

Snout is written in Rust. With a recent Rust toolchain installed:

    cargo build --release

The compiled binary will be available at `target/release/snout`.

## Roadmap

- [x] Plain-text search across a folder
- [x] Recursive search into subfolders
- [~] Document formats: DOCX (done), PDF and XLSX (in progress)
- [ ] Full-text index for fast search over large file sets
- [ ] Semantic search (find by topic, not just exact words)
- [ ] Image content search
- [ ] Desktop app

## License

MIT © Blazi2002
