# FreeBSD-fs-crypt

**Version:** 0.1.4  
**License:** MIT  
**Author:** [Afshin Tavahin](mailto:afshin@tavahin.com)  
**Repository:** [GitHub - TavahinAfshin/FreeBSD-fs-crypt](https://github.com/TavahinAfshin/FreeBSD-fs-crypt)

## Overview

**FreeBSD-fs-crypt** is a kernel module for FreeBSD that enables **transparent filesystem encryption** using the **Rust** programming language. It automatically encrypts data written to disk and decrypts data read from disk — without requiring user intervention. This module is designed to enhance privacy and security at the system level, while remaining invisible to applications and users.

> All filesystem I/O remains transparent — users interact with files normally, while encryption and decryption happen behind the scenes.

---

## ✨ Features

- 🔐 **AES-256-GCM Encryption** for strong authenticated encryption
- 🔑 **Secure Key Management** for generating, storing, and retrieving encryption keys
- ⚙️ **Transparent VFS Integration** — no change to user workflows
- 🚫 **Tamper Protection** using authenticated encryption
- 🦀 **Memory Safety** through Rust
- 🧠 **Cross-Platform Development Support** (FreeBSD target with macOS/Windows dev support)

---

## 🧱 Architecture

The module is built on a layered architecture:

### 1. Kernel Integration Layer
Interfaces with FreeBSD’s kernel via **FFI (Foreign Function Interface)** to hook into system-level operations.

### 2. Cryptography Module
Implements **AES-256-GCM** using Rust’s cryptography libraries for encryption and decryption.

### 3. Key Management System
Handles:
- Secure key generation using strong CSPRNGs
- In-memory key storage (with optional hardware backing in future versions)
- Support for future post-quantum crypto integration

### 4. VFS Hooks
Intercepts file read and write operations at the **Virtual File System (VFS)** layer to apply encryption and decryption logic.

---

## ⚙️ Development & Usage

- ✅ **Target OS**: FreeBSD (13+)
- 💻 **Dev Platforms**: macOS, Windows, FreeBSD
- 🔧 **Language**: Rust (2021 Edition) with `bindgen` and `libc` FFI integration
- 📦 Built with conditional compilation to isolate platform-specific logic

---

## 🚧 Status and Roadmap

### Current Status: `In Development`

### Roadmap:
- [x] VFS operation interception (basic prototype)
- [x] AES-256-GCM encryption/decryption engine
- [x] Basic key management layer
- [ ] Full integration into FreeBSD kernel module interface
- [ ] Extensive testing with edge cases
- [ ] Security audits and performance benchmarking
- [ ] Post-quantum cryptography support (research phase)
- [ ] Open source release with documentation

---

## 🔐 Security Considerations

- Uses **authenticated encryption** to ensure both confidentiality and integrity
- Minimizes memory safety issues via Rust’s ownership model
- Plans to integrate **post-quantum encryption** in future releases
- Focus on high-performance I/O encryption with minimal overhead

---

## 📢 Contributions

Contributions are welcome once the initial prototype stabilizes. Planned areas for contribution:

- Testing suite and fuzzing
- Performance improvements
- Cryptographic algorithm alternatives
- Documentation and user guides

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

## 🙋‍♂️ Contact

**Afshin Tavahin**  
📧 [afshin@tavahin.com](mailto:afshin@tavahin.com)  
🌐 [GitHub: TavahinAfshin](https://github.com/TavahinAfshin)

---

> _Secure the filesystem, transparently._  
> — FreeBSD-fs-crypt

