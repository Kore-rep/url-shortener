FROM rust:1.76.0
COPY ./ ./ 
ENV RUST_LOG=info
RUN cargo build --release
CMD ["./target/release/url-shortener-api"]
