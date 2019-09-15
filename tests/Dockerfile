FROM rust

# Create /src as root
RUN mkdir /src && chmod -R a+w /src

# Create non-root user
RUN useradd -ms /bin/bash nonroot
USER nonroot

# Install Node.js
ENV NODE_VERSION 12.0.0
ENV NVM_DIR /home/nonroot/.nvm
ENV PATH $NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH
RUN curl https://raw.githubusercontent.com/creationix/nvm/master/install.sh | bash
# Check that Node.js was correctly installed
RUN node --version

# Install Go
ENV GO_VERSION 1.12.1
ENV GOENV_ROOT /home/nonroot/.goenv
ENV PATH $GOENV_ROOT/bin:$GOENV_ROOT/shims:$PATH
RUN git clone https://github.com/syndbg/goenv.git $GOENV_ROOT \
  && eval "$(goenv init -)" \
  && goenv install $GO_VERSION \
  && goenv global $GO_VERSION \
  && chmod -R a+x $GOENV_ROOT
# Check that Go was correctly installed
RUN go version

# Install Ruby
ENV RUBY_VERSION 2.6.3
ENV RBENV_ROOT /home/nonroot/.rbenv
ENV PATH $RBENV_ROOT/bin:$RBENV_ROOT/shims:$PATH
RUN curl -fsSL https://github.com/rbenv/rbenv-installer/raw/master/bin/rbenv-installer | bash \
  && rbenv install $RUBY_VERSION \
  && rbenv global $RUBY_VERSION
# Check that Ruby was correctly installed
RUN ruby --version

# Install Python
ENV PYTHON_VERSION 3.6.9
ENV PYENV_ROOT /home/nonroot/.pyenv
ENV PATH $PYENV_ROOT/bin:$PYENV_ROOT/shims:$PATH
RUN curl https://pyenv.run | bash \
  && pyenv install $PYTHON_VERSION \
  && pyenv global $PYTHON_VERSION \
  && chmod -R a+x $PYENV_ROOT
# Check that Python was correctly installed
RUN python --version

# Create blank project
RUN USER=nonroot cargo new --bin /src/starship
WORKDIR /src/starship

# We want dependencies cached, so copy those first
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Cargo.toml will fail to parse without my_benchmark
RUN mkdir benches
RUN touch benches/my_benchmark.rs

# This is a dummy build to get dependencies cached
RUN cargo build --release \
  && rm -rf src target/debug/starship*

# "-Z unstable-options" is required for "--include-ignored"
CMD ["cargo", "test", "--", "-Z", "unstable-options", "--include-ignored"]
