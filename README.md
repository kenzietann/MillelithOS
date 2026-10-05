<p align="center">
  <img src="assets/branding/millelith-os-logo.png" alt="Millelith OS logo" width="720">
</p>

# Millelith OS

<p align="center">
  <a href="#progress"><img src="https://img.shields.io/badge/status-development-2563eb?style=flat-square" alt="Status: development"></a>
  <a href="#what-i-want-to-learn"><img src="https://img.shields.io/badge/language-Rust-DEA584?style=flat-square&amp;logo=rust&amp;logoColor=white" alt="Primary language: Rust"></a>
  <a href="#the-plan"><img src="https://img.shields.io/badge/platform-amd64-2563eb?style=flat-square" alt="Initial platform: amd64"></a>
  <a href="#the-plan"><img src="https://img.shields.io/badge/boot-UEFI_native-2ea44f?style=flat-square" alt="Boot: UEFI native"></a>
  <a href="#the-plan"><img src="https://img.shields.io/badge/type-Unix_OS-f97316?style=flat-square" alt="Type: Unix OS"></a>
  <a href="https://github.com/kenzietann/MillelithOS/stargazers"><img src="https://img.shields.io/github/stars/kenzietann/MillelithOS?style=flat-square&amp;logo=github&amp;logoColor=white&amp;color=2563eb" alt="GitHub stars"></a>
</p>

I'm building Millelith OS to learn how computers and operating systems work from the ground up. I want to understand what happens after code compiles: how firmware boots, how operating systems coordinate processes, and how software talks to hardware.

Rather than being confined to an emulator-only toy OS with no drivers, Millelith OS follows the proven architectural model of modern systems like Android and macOS:
1. **Firmware & Boot (100% Rust):** A custom, zero-dependency bare-metal UEFI bootloader written in Rust from scratch.
2. **The Hardware Engine:** The official Linux kernel (`vmlinuz`) driving physical PC hardware (Wi-Fi, 3D GPU, NVMe, USB 3.0, and multi-core CPU scheduling).
3. **The Operating System & Userspace (100% Rust):** A custom Unix userspace built in Rust, featuring a custom PID 1 Init system (`/sbin/init`), an interactive command-line shell with syntax parsing and job control, and memory-safe core utilities.

The earlier C++ UEFI proof-of-concept is preserved in the [`archive/cpp-uefi`](https://github.com/kenzietann/MillelithOS/tree/archive/cpp-uefi) branch.

For the detailed technical specifications and milestone roadmap, see [PRD.md](PRD.md).

---

## The Architecture

```
+-------------------------------------------------------------------------+
|                        Millelith OS Architecture                         |
+-------------------------------------------------------------------------+
| [Layer 3: Userspace & Apps (100% Custom Rust)]                          |
|  - Millelith Init (PID 1): Mounts /proc, /sys, /dev, manages services   |
|  - Millelith Shell: Interactive CLI, AST parser, job control, pipelines |
|  - Millelith Coreutils: Essential Unix utilities in memory-safe Rust    |
+-------------------------------------------------------------------------+
                                    │ POSIX Syscalls (fork, exec, pipe, ioctl)
+-----------------------------------v-------------------------------------+
| [Layer 2: The Hardware Engine (The Linux Kernel - vmlinuz)]             |
|  - Universal Hardware Support: Wi-Fi 6, 3D GPU (DRM/KMS), NVMe, USB 3.0 |
|  - Virtual Memory (Paging), Hardware Ring 0 Protection, CPU Scheduler  |
+-------------------------------------------------------------------------+
                                    │ Linux 64-bit Boot Protocol / EFI Handover
+-----------------------------------v-------------------------------------+
| [Layer 1: Custom Rust UEFI Bootloader (crates/bootloader)]              |
|  - Zero-dependency bare-metal PE32+ executable (x86_64-unknown-uefi)    |
|  - Probes Graphics Output Protocol (GOP) for high-res splash screen     |
|  - Locates and loads /boot/vmlinuz and /boot/initrd from FAT32/EXT4     |
|  - Sets up boot parameters and transfers control to the kernel          |
+-------------------------------------------------------------------------+
```

---

## What I Want to Learn

- **Firmware & Bare-Metal Systems:** 64-bit UEFI calling conventions (`extern "efiapi"`), PE/COFF execution, raw pointers, memory alignment, and GOP linear framebuffer rendering.
- **Operating System Core Concepts:** The POSIX system call interface (`fork`, `execve`, `mmap`, `pipe`, `ioctl`), signals (`SIGINT`, `SIGCHLD`), file descriptors, and virtual filesystems (`/proc`, `/sys`, `/dev`).
- **PID 1 & System Lifecycle:** Building an Init system from scratch to manage process trees, reap orphan processes, and handle system shutdown.
- **Compilers & Parsers:** Designing a custom shell with lexical analysis, AST parsing, and command execution pipelines.

---

## Progress

### 1. Proof of Concept (C++ UEFI)
- [x] Boot an x86-64 EFI application in QEMU.
- [x] Print text via UEFI firmware console services.
- [x] Acquire the Graphics Output Protocol (GOP) linear framebuffer.
- [x] Verify pixel formats and fill the screen with `#18416E` blue.
- [x] *Archived to branch [`archive/cpp-uefi`](https://github.com/kenzietann/MillelithOS/tree/archive/cpp-uefi).*

### 2. Phase 1: The Rust UEFI Bootloader (`crates/bootloader`)
- [x] Pure zero-dependency runtime implemented (`#![no_std]`, `#[panic_handler]`, `EfiStatus`).
- [x] Modular architecture (`src/uefi.rs` for protocol definitions, `src/main.rs` for boot logic).
- [x] Booted in QEMU with firmware text console output (`Hello from Millelith OS!`).
- [x] Implement `LocateProtocol` and query Graphics Output Protocol (GOP).
- [x] Verify linear framebuffer rendering: Fill screen with Millelith Red.
- [ ] Implement file system reading to load `vmlinuz` and `initrd` into physical memory.
- [ ] Hand off execution to Linux kernel via 64-bit EFI boot protocol.

### 3. Phase 2: Millelith Init (PID 1 in Rust)
- [ ] Implement standalone freestanding Rust PID 1 binary (`/sbin/init`).
- [ ] Mount `/proc`, `/sys`, and `/dev` virtual filesystems.
- [ ] Implement signal handling and orphan process reaping.
- [ ] Spawn the primary shell session.

### 4. Phase 3: Millelith Shell & Core Utilities
- [ ] Interactive REPL with syntax parsing and AST generation.
- [ ] Process pipeline execution using `fork()`, `execve()`, and `pipe()`.
- [ ] File redirection (`<`, `>`, `>>`).
- [ ] Memory-safe Unix core utilities (`ls`, `cat`, `ps`, `kill`, `uname`).

---

## Artwork

<img src="assets/branding/millelith-os-icon.png" alt="Millelith OS icon" width="96">

- [Icon](assets/branding/millelith-os-icon.png)
- [Logo](assets/branding/millelith-os-logo.png)

---

## Toolchain & Requirements

### Host Dependencies
- **Rust (2024 Edition):** `rustup default nightly` (or stable 1.85+)
- **Rust Target:**
  ```sh
  rustup target add x86_64-unknown-uefi
  ```
- **QEMU:** `brew install qemu` (macOS) or `sudo apt install qemu-system-x86` (Linux)

---

## References

- [UEFI Specification 2.10](https://uefi.org/specifications)
- [The Linux/x86 Boot Protocol](https://docs.kernel.org/arch/x86/boot.html)
- [OSDev Wiki: UEFI](https://wiki.osdev.org/UEFI)
- [The Linux Programming Interface (Michael Kerrisk)](https://man7.org/tlpi/)
- [OSDev Wiki: APIC](https://wiki.osdev.org/APIC)