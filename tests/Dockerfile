FROM rust

# Install Tarpaulin (code coverage tool)
# https://github.com/xd009642/tarpaulin
COPY --from=xd009642/tarpaulin /usr/local/cargo/bin/cargo-tarpaulin /usr/local/cargo/bin/cargo-tarpaulin

# Install Node.js
# Copied from https://github.com/nodejs/docker-node/blob/master/12/stretch/Dockerfile
ENV NODE_VERSION 12.0.0
RUN curl -fsSLO --compressed "https://nodejs.org/dist/v$NODE_VERSION/node-v$NODE_VERSION-linux-x64.tar.xz" \
	&& tar -xJf "node-v$NODE_VERSION-linux-x64.tar.xz" -C /usr/local --strip-components=1 --no-same-owner \
	&& rm "node-v$NODE_VERSION-linux-x64.tar.xz" \
# Check that Node.js was correctly installed
	&& node --version \
	&& npm --version

# Install Go
# Copied from https://github.com/docker-library/golang/blob/master/1.12/stretch/Dockerfile
ENV GOLANG_VERSION 1.10
RUN wget -O go.tgz "https://dl.google.com/go/go${GOLANG_VERSION}.linux-amd64.tar.gz" \
	&& tar -C /usr/local -xzf go.tgz \
	&& rm go.tgz \
	&& export PATH="/usr/local/go/bin:$PATH" \
	# Check that Go was correctly installed
	&& go version

# Install Python
# Copied from https://github.com/docker-library/python/blob/master/3.7/stretch/Dockerfile
ENV PYTHON_VERSION 3.6.8
RUN wget -O python.tar.xz "https://www.python.org/ftp/python/${PYTHON_VERSION%%[a-z]*}/Python-$PYTHON_VERSION.tar.xz" \
	&& mkdir -p /usr/src/python \
	&& tar -xJC /usr/src/python --strip-components=1 -f python.tar.xz \
	&& rm python.tar.xz \
	\
	&& cd /usr/src/python \
	&& gnuArch="$(dpkg-architecture --query DEB_BUILD_GNU_TYPE)" \
	&& ./configure --enable-optimizations \
	&& make -j8 \
	&& sudo make altinstall \
	\
	&& find /usr/local -depth \
		\( \
			\( -type d -a \( -name test -o -name tests \) \) \
			-o \
			\( -type f -a \( -name '*.pyc' -o -name '*.pyo' \) \) \
		\) -exec rm -rf '{}' + \
	&& rm -rf /usr/src/python \
	\
	&& ln -s /usr/local/bin/python3 /usr/local/bin/python \
# Check that Python was correctly installed
	&& python --version

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

CMD ["cargo", "tarpaulin", "--ignored", "--out", "Xml"]
