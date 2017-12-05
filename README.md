# tweet-at-compile-time
コンパイル時にツイートするマクロ

## Requirements
* Rust +nightly 1.23
* Twitter の consumer token と access token

## Usage

1. `examples/token.json` を作成し，キーの値を保存しておく
2. `examples/hello.rs` を弄る
3. `cargo build --example hello`
