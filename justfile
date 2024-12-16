#!/usr/bin/env -S just --justfile

default:
    cargo nextest run --no-fail-fast
