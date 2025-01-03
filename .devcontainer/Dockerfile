FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    #
    # Install
    && apt-get -y install \
    python3-pip \
    cmake clang \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Install dependency manager Conan
RUN pip3 install conan \
    && conan profile detect
