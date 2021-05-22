####################
FROM ubuntu AS dpdk
####################
# Put the actual compilation in its own stage for speedy reuse

RUN apt update \
    && apt install -y \
    build-essential \
    curl \
    libnuma-dev \
    libpcap-dev \
    meson \
    ninja-build \
    python3-pyelftools 

WORKDIR /usr/local/src/dpdk
RUN curl http://fast.dpdk.org/rel/dpdk-19.11.6.tar.xz --output - \
    | tar --extract --xz --verbose --strip-components=1

RUN meson -Dexamples=all build
WORKDIR /usr/local/src/dpdk/build
RUN ninja 
RUN ninja install


####################
FROM ubuntu as rust
####################
# Now build the dev env

RUN apt update \
    && apt install -y \
    build-essential \
    curl 

RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf \
    | bash -s -- -y

RUN /root/.cargo/bin/rustup component add \
    rls \ 
    rust-analysis \
    rust-src 


##################
FROM ubuntu as dev
##################

COPY --from=dpdk /usr/local/ /usr/local/
RUN ldconfig

COPY --from=rust /root/.rustup /root/.rustup
COPY --from=rust /root/.cargo /root/.cargo
ENV PATH /root/.cargo/bin:${PATH}

RUN apt update \
    && \
    DEBIAN_FRONTEND="noninteractive" \
    TZ="Europe/London" \
    apt install -y \
    build-essential \
    clang \
    curl \
    git \
    iproute2 \ 
    iputils-ping \
    libhugetlbfs-bin \
    libhugetlbfs-dev \
    libpcap-dev \
    libssl-dev \
    libnuma-dev \
    meson \
    ninja-build \
    pkg-config \
    vim 

RUN apt update \
    && apt install -y \
    python3-pip

RUN curl -sSL https://raw.githubusercontent.com/python-poetry/poetry/master/get-poetry.py \
    | python3 -


RUN cargo install \
    cargo-edit
