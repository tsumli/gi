FROM ubuntu:24.04

RUN apt update && apt upgrade -y
RUN apt install -y \
    curl \
    libssl-dev \
    gcc \
    git \
    pkg-config \
    vim

# modify permission
ARG USER
ARG GROUP=${USER}
ARG UID=1000
ARG GID=1000
# Check if the group exists before creating it
RUN getent group ${GID} || groupadd -g ${GID} ${GROUP}
# Create the user 
RUN useradd -m -u ${UID} -g ${GID} -s /bin/bash ${USER}

RUN echo "$USER:$GID" | chpasswd && \
    usermod --shell /bin/bash $USER && \
    usermod -aG sudo $USER && \
    mkdir /etc/sudoers.d && \
    echo "$USER ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers.d/$USER && \
    chmod 0440 /etc/sudoers.d/$USER && \
    usermod  --uid $UID $USER && \
    groupmod --gid $GID $USER

RUN apt install -y sudo 
USER ${USER}
ENV HOME=/home/${USER}

# install rust
RUN curl https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y
RUN echo "source $HOME/.cargo/env" >> $HOME/.bashrc
ENV PATH=/home/${USER}/.cargo/bin:${PATH}
RUN rustup update
RUN rustup install nightly
RUN rustup component add clippy rls rust-analysis rust-src rust-docs rustfmt rust-analyzer
