FROM rust:latest

# Create blank project
RUN USER=root cargo new --bin starship
WORKDIR /starship

# We want dependencies cached, so copy those first
COPY ./Cargo.lock ./Cargo.lock 
COPY ./Cargo.toml ./Cargo.toml 

# Cargo.toml will fail to parse without my_benchmark
RUN mkdir benches
RUN touch benches/my_benchmark.rs

# This is a dummy build to get dependencies cached
RUN cargo build --release
RUN rm src/*.rs

# Now copy in the rest of the source files
COPY ./src ./src
COPY ./tests ./tests

CMD [ "cargo", "test", "--", "--ignored"]
