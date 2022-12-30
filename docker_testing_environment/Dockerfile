FROM python:3.10.8-slim

RUN apt-get update && apt-get install -y build-essential curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && /root/.cargo/bin/rustup default stable
ENV PATH="/root/.cargo/bin:${PATH}" RUSTUP_HOME="/root/.rustup"

# requirements.txt is generated with pip-comile.
# See the justfile at the repository root.
COPY requirements.txt /deps/requirements.txt

RUN python -m venv /deps/venv && . /deps/venv/bin/activate && pip install -r /deps/requirements.txt
