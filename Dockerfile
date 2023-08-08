FROM rust:bookworm
LABEL Name=rustaudio Version=0.0.1

RUN apt-get update

RUN apt-get -f install libasound2-dev -y
RUN apt-get -f install libpango1.0-dev -y
RUN apt-get -f install libatk1.0-dev -y
RUN apt-get -f install libgtk-3-dev -y

RUN USER=root cargo new --bin rust_audio
WORKDIR /rust_audio

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./test ./test

RUN rm ./target/release/deps/rust_audio*
RUN cargo install --path .

CMD ["rust_audio"]
