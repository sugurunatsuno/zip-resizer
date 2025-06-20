# ZIP画像リサイズツール

本リポジトリは、ZIP ファイル内に含まれる画像を一括でリサイズし、再圧縮した ZIP を生成するデスクトップアプリケーションです。Rust 製の高速コア処理と Tauri による GUI を組み合わせており、ドラッグ＆ドロップで複数ファイルを指定して簡単に画像を軽量化できます。

## 特長
- JPEG/PNG 画像を指定した最大幅・最大高さ・品質でリサイズ
- 複数の ZIP ファイルをまとめてドラッグ＆ドロップで処理
- 処理状況を画面上で確認可能
- クロスプラットフォーム（Windows/Mac/Linux）対応

## 依存環境
- [Rust](https://www.rust-lang.org/) 1.70 以上
- [Node.js](https://nodejs.org/) と [pnpm](https://pnpm.io/)

## 実行方法
1. 依存パッケージをインストールします。
   ```sh
   pnpm install
   ```
2. アプリケーションを起動します。
   ```sh
   pnpm tauri dev
   ```

## CLI について
`src-tauri/src/cli.rs` にコマンドライン版のサンプル実装がありますが、Cargo.toml にはバイナリとして登録されていません。必要に応じて `[[bin]]` を追記してビルドしてください。

## ライセンス
OSS を想定しています（詳細未定）。

フィードバックや要望は Issue/Pull Request にてお知らせください。
