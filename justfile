# list available recipes
default:
  @just --list

# install dev dependencies
dev-env:
  pip install maturin
  maturin develop --extras=dev

# apply formatters to the code
fix:
  black .
  isort .
  cargo fmt

# check the code for errors
check:
  #!/usr/bin/env bash

  error=0
  trap error=1 ERR

  echo
  ( set -x; black --check . )
  echo
  ( set -x; isort --check . )
  echo
  ( set -x; flake8 . )
  echo
  ( set -x; mypy . )
  echo
  ( set -x; pytest )
  echo
  ( set -x; cargo check )
  echo
  ( set -x; cargo fmt --check )
  echo
  ( set -x; cargo clippy -- -D warnings)

  test $error = 0

# build the docker testing environment
build-testing-environment:
  pip-compile -o docker_testing_environment/requirements.txt --extra dev pyproject.toml
  docker image build -t flour-testing-environment:latest docker_testing_environment

# enter the docker testing environment
docker:
  docker run -it --rm --mount type=bind,source="$(pwd)",target=/code flour-testing-environment /bin/sh

# run benchmarks
benchmarks:
  pytest benchmarks

# build the library
build:
  maturin develop --profile=release --extras=dev
