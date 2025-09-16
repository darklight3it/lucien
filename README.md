# Lucien

A rust cli application to fix ebook metadata with authoritative online data.

## Todo

- [ ] Load ebook metadata from a book
- [ ] Build a client for open library
- [ ] Build a cli to do that. Commands are:
  - parse ebook metadata
  - update


## Ideas

- [ ] hooks on a folder. automatic librarian
- [ ] Change the name to ai librarian.


```
file system example

llmender/
├─ Cargo.toml
├─ Cargo.lock
├─ README.md
├─ models/
│  └─ ggml-model-7b.bin
├─ ebooks/
├─ src/
│  ├─ main.rs
│  ├─ lib.rs
│  ├─ cli.rs
│  ├─ ebook/
│  │  ├─ mod.rs
│  │  ├─ epub.rs
│  │  ├─ pdf.rs
│  │  └─ mobi.rs
│  ├─ llama/
│  │  ├─ mod.rs
│  │  └─ inference.rs
│  └─ types.rs
├─ examples/
│  └─ test_ebook.rs
└─ scripts/
```
