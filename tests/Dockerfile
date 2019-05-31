FROM rust

# Install Tarpaulin (code coverage tool)
# https://github.com/xd009642/tarpaulin
COPY --from=xd009642/tarpaulin /usr/local/cargo/bin/cargo-tarpaulin /usr/local/cargo/bin/cargo-tarpaulin

# Install Node.js
ENV NODE_VERSION 12.0.0
ENV PATH /root/.nvm/versions/node/v$NODE_VERSION/bin:$PATH
RUN curl https://raw.githubusercontent.com/creationix/nvm/master/install.sh | bash
# Check that Node.js was correctly installed
RUN node --version

# Install Go
ENV GO_VERSION 1.10.0
ENV GOENV_ROOT /root/.goenv
ENV PATH $GOENV_ROOT/bin:$GOENV_ROOT/shims:$PATH
RUN git clone https://github.com/syndbg/goenv.git $GOENV_ROOT \
	&& eval "$(goenv init -)" \
	&& goenv install $GO_VERSION \
	&& goenv global $GO_VERSION
# Check that Go was correctly installed
RUN go version

# Install Python
ENV PYTHON_VERSION 3.6.8
ENV PYENV_ROOT /root/.pyenv
ENV PATH $PYENV_ROOT/bin:$PYENV_ROOT/shims:$PATH
RUN curl https://pyenv.run | bash \
  && pyenv install $PYTHON_VERSION \
  && pyenv global $PYTHON_VERSION
# Check that Python was correctly installed
RUN python --version

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

# Delete the dummy build
RUN rm -rf /starship

# Create the directory for the real source files
RUN mkdir starship
WORKDIR /starship

# Instrument coverage tooling and run the full test suite
CMD ["cargo", "tarpaulin", "--ignored", "--ignore-tests", "--out", "Xml"]
