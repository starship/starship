FROM rust:1.34.0

# Install Node.js
ENV NODE_VERSION 12.0.0
RUN curl https://raw.githubusercontent.com/creationix/nvm/master/install.sh | bash \
  && . $HOME/.nvm/nvm.sh \
  && nvm install $NODE_VERSION \
  && nvm alias default $NODE_VERSION \
  && nvm use default
ENV PATH /root/.nvm/versions/node/v$NODE_VERSION/bin:$PATH
# Check that Node.js was correctly installed
RUN node --version 

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
RUN rustup install stable
RUN rustup run stable cargo build --release

# Delete the dummy build
RUN rm -rf /starship

# Create the directory for the real source files
RUN mkdir starship
WORKDIR /starship

# Run with rustup to use stable instead of the Rust default
CMD [ "rustup", "run", "stable", "cargo", "test", "--", "--ignored"]
