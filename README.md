# FreeBSD‑fs‑crypt

**Version 0.1.4 – MIT License**  
*A transparent filesystem‑encryption kernel module for FreeBSD, written in Rust.*

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build status](https://img.shields.io/badge/FreeBSD-kmod-orange)](#building)
[![Rust Edition](https://img.shields.io/badge/Rust-2021-blue)](https://www.rust-lang.org/)

---

## Table of Contents
1. [Why FreeBSD‑fs‑crypt?](#why-freebsd-fs-crypt)
2. [Features](#features)
3. [Architecture](#architecture)
4. [Quick Start](#quick-start)
5. [Building the Module](#building)
6. [Key Management](#key-management)
7. [Performance Notes](#performance-notes)
8. [Roadmap](#roadmap)
9. [Contributing](#contributing)
10. [License](#license)

---

## Why FreeBSD‑fs‑crypt?
While ZFS and GELI provide powerful encryption options, they operate at the pool or
block‑device level. **FreeBSD‑fs‑crypt** focuses on *per‑file* transparency:
applications read and write plaintext, while on‑disk data remain AES‑256‑GCM
encrypted—no user intervention required. Built entirely in Rust, the project
leverages memory‑safety guarantees while interfacing with the FreeBSD kernel
through an ergonomic FFI layer.

---

## Features
| Capability                        | Status | Notes |
|----------------------------------|:------:|-------|
| Transparent read/write encryption| ✅     | VFS hooks wrap every `VOP_READ` / `VOP_WRITE` |
| AES‑256‑GCM (authenticated)      | ✅     | Provides confidentiality & integrity |
| Secure key generation & storage  | ✅     | Per‑filesystem master key + per‑file nonces |
| Rust → Kernel FFI safety layer   | ✅     | `#![no_std]` + `kernel_shim` crate |
| Conditional compilation (macOS/Win dev) | ✅ | Build‑time `cfg` gates |
| Post‑quantum cipher plug‑in      | 🚧     | Kyber integration slated for v0.2 |

---

## Architecture

```text
          +---------------------------+
          |   User Applications       |
          +-------------+-------------+
                        |
                  VFS Calls (read/write/open)
                        |
         +--------------v---------------+
         |  FreeBSD‑fs‑crypt VFS Hooks  |
         +--+------------------------+--+
            |                        |
   +--------v--------+      +--------v--------+
   |  Crypto Module  |      |  Key Manager    |
   |  (AES‑256‑GCM)  |      |  (kernel vault) |
   +--------+--------+      +--------+--------+
            |                        |
   +--------v--------+      +--------v--------+
   |   FreeBSD VFS   |      |  Entropy Source |
   +-----------------+      +-----------------+
