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

# Install Go
ENV GO_VERSION 1.10.0
ENV GOENV_ROOT /root/.goenv
ENV GO_ROOT /root/go
# RUN git clone https://github.com/wfarr/goenv.git $GOENV_ROOT
RUN git clone https://github.com/syndbg/goenv.git $GOENV_ROOT
ENV PATH $GOENV_ROOT/bin:$GOENV_ROOT/shims:$PATH
RUN eval "$(goenv init -)"
RUN mkdir -p $GO_ROOT
ENV GOPATH $GO_ROOT
ENV PATH $GO_ROOT/bin:$PATH
RUN goenv install $GO_VERSION
RUN goenv global $GO_VERSION
# Check that Go was correctly installed

RUN go version
# Install Python
ENV PYTHON_VERSION 3.7.3
ENV PYENV_ROOT /root/.pyenv
ENV PATH $PYENV_ROOT/shims:$PYENV_ROOT/bin:$PATH
RUN curl https://pyenv.run | bash \
  && pyenv update \
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
RUN rustup install stable
RUN rustup run stable cargo build --release

# Delete the dummy build
RUN rm -rf /starship

# Create the directory for the real source files
RUN mkdir starship
WORKDIR /starship

<<<<<<< HEAD
# Run with rustup to use stable instead of the Rust default
CMD [ "rustup", "run", "stable", "cargo", "test", "--", "--ignored"]
=======
CMD ["cargo", "test", "--", "--ignored"]
>>>>>>> Add integration tests for Python segment
