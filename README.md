# vimparq

Edit Apache Parquet files directly in Vim using a Rust-based CLI tool that converts Parquet to and from JSONL format.

## 🔧 Requirements

- Vim or Neovim
- [Rust](https://www.rust-lang.org/tools/install)

## 🚀 Installation

### With Vim 8+ native packages

```bash
mkdir -p ~/.vim/pack/plugins/start
cd ~/.vim/pack/plugins/start
git clone https://github.com/taylordeckard/vimparq.git
cd vimparq/rust
cargo install --path .
```

## Usage

This will convert the Parquet file into a temporary JSONL file which can be edited with Vim.
```sh
vim ./my.parquet
```
Upon saving, the original parquet file will be updated with the changes made in Vim.

Parquet files can also be created from scratch.
```sh
vim new.parquet
```
As long as the contents are valid JSONL, the new parquet file will be created upon saving.
