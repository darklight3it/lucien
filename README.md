# llmender
A rust cli application to fix ebook metadata with LLMs


```
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