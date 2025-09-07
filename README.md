# llmender
A rust cli application to fix ebook metadata with LLMs

## Todo

- [x] Script to download and update llama model.
- [ ] Try playing with it.
- [ ] Test with a mock up json model.
- [ ] Unit Testing Strategy
- [ ] Check why it is not able to load the model.
- [ ] Research how to package models in single applications


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