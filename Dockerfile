# Dockerfile for Rust Development
FROM ubuntu:20.04

# Set up environment variables
ARG VITIS_FILE=Vitis_Embedded_Lin_2024.1_0522_2023
ARG VITIS_VERSION=2024.1

# Add environment variables and arguments to solve for tzdata issue
# https://serverfault.com/questions/949991/how-to-install-tzdata-on-a-ubuntu-docker-image
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=America/New_York

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    python3 \
    python3-pip \
    libncurses5-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install ARM toolchain for cross-compilation
RUN apt-get update && apt-get install -y \
    libc6-dev-armhf-cross \
    gcc-arm-none-eabi \
    gdb-multiarch \
    && rm -rf /var/lib/apt/lists/*

# Install LLVM and Clang
RUN apt-get update && apt-get install -y \
    llvm \
    clang \
    lld \
    && rm -rf /var/lib/apt/lists/*

# Install Vitis
WORKDIR /tmp
ADD . /tmp

RUN cd /tmp && \
    tar -xvf ${VITIS_FILE}.tar.gz && \
    cd ${VITIS_FILE} && \
    ./xsetup --agree 3rdPartyEULA,XilinxEULA --batch Install --edition "Vitis Embedded Development" --location "/opt/xilinx" . && \
    ./installLibs.sh

# Solve no locales bug in XSCT no display
RUN apt-get install locales && \
    locale-gen en_US.UTF-8 && \
    update-locale LANG=en_US.UTF-8

# Install Rust components
RUN cargo install justfile
RUN cargo install cargo-binutils
RUN rustup component add rust-src
RUN rustup default nightly

# Add to PATH
ENV PATH ${PATH}:/opt/xilinx/Vitis/${VITIS_VERSION}/bin:/opt/xilinx/xrt/bin
ENV XILINX_VITIS /opt/xilinx/Vitis/${VITIS_VERSION}

# Clean up
RUN rm -rf /tmp/*

# Set up a volume to mount the local windows directory to the workspace
VOLUME /workspace

# Default working directory
WORKDIR /workspace

# Ports
EXPOSE 3121
EXPOSE 3000

# Default command
CMD ["/bin/bash"]