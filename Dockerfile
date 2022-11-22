FROM alpine:3.16
RUN apk update 
RUN apk add curl git libgcc gcc musl-dev openssl-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN git clone https://github.com/ayourtch/async-rust-test-temp
WORKDIR async-rust-test-temp
ENV PATH="/root/.cargo/bin:$PATH"
RUN cargo build 
RUN apk add gdb
CMD gdb target/debug/litir


