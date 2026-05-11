# rust-tutorial
Rust勉強用

## 参考
- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/title-page.html)

## cargo
```sh
# プロジェクト作成
cargo new <プロジェクト名>

# プロジェクト作成（プロジェクトルートのディレクトリ作成済みの場合）
cargo init

# ビルド
cargo build
# リリースビルド
cargo build --release

# (ビルド +)実行
cargo run

# 実行ファイルを生成せずにビルドを実行
# コンパイルエラーの有無を高速に確認できる
cargo check
```