# Rustの公式イメージを使用
FROM rust:latest

# 作業ディレクトリを作成
WORKDIR /app

# 必要なファイルをコピー
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# 依存関係を解決 & ビルド
RUN cargo build --release

# 実行ファイルをコピー
CMD ["./target/release/docker_web"]

