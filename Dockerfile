FROM rustlang/rust:nightly-slim

# FROM rust

WORKDIR /usr/src/califications_CUD

COPY . .

EXPOSE 8000

# COPY .cargo ~/.cargo

# RUN cargo install --path .

RUN curl http://google.com

RUN cargo build -r
# RUN HTTP_PROXY=http://ntlm-proxy.org.com:8080
# RUN HTTPS_PROXY=http://ntlm-proxy.org.com:8080
# RUN PROXY_USER=NT_DOMAIN\username
# RUN PROXY_PASS=the_password



CMD ["./target/debug/./califications_CUD"]

# CMD ["cargo", "run"]