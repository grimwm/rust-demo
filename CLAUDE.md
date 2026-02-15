# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

This project uses recursive Makefiles with `rustc` directly (no Cargo).

- **Build all:** `make build` (from repo root)
- **Clean all:** `make clean` (from repo root)
- **Build a single example:** `make -C hello/single build` or `make -C hello/multi build`

## Architecture

This is a collection of Rust demo programs compiled directly with `rustc` (not Cargo). The build system is nested Makefiles.

- `hello/single/` — Standalone single-file hello world (`hello.rs`)
- `hello/multi/` — Multi-file example demonstrating modules: `hello.rs` (entry point with `mod demo`) calls into `demo.rs` (provides `pub fn hello()`)

The root Makefile delegates to `hello/Makefile`, which in turn delegates to each example's Makefile.
