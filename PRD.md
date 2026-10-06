# Product Requirements Document (PRD): Millelith OS (Unix Edition)

**Project Name:** Millelith OS  
**Status:** Phase 1 Bootloader Development  
**Author & Lead Developer:** Kenzie Tann  
**Core Technologies:** Rust (`crates/bootloader`, `crates/init`, `crates/shell`), Linux Kernel (`vmlinuz`), POSIX/Unix Architecture  
**Core Purpose:** An operating system engineered from scratch to master Computer Science in a single, comprehensive project. Built to be a daily-drivable, physical-PC-ready Unix operating system combining a **custom bare-metal Rust UEFI bootloader**, the **Linux kernel as the hardware engine**, and a **100% custom Rust userspace and init system**.

---

## 1. Executive Summary & Architectural Vision

### 1.1 The Hybrid Architectural Model
Writing modern hardware drivers (5G WiFi, Nvidia/AMD 3D GPU pipelines, USB 3.2 xHCI) from scratch takes decades for multi-billion-dollar teams. Rather than being trapped in an emulator-only toy OS that cannot connect to the internet, **Millelith OS adopts the proven industry architecture used by Android, macOS, and ChromeOS**:

1. **Firmware & Boot (100% Custom Rust):**
   - A zero-dependency bare-metal UEFI bootloader (`crates/bootloader`) built from scratch in Rust, responsible for firmware handoff, GOP graphical display, kernel discovery, and execution.
2. **The Hardware Engine (The Linux Kernel):**
   - The battle-tested Linux kernel (`vmlinuz`) drives modern physical PC hardware, providing rock-solid WiFi, GPU hardware acceleration, Bluetooth, audio, multi-core scheduling, and POSIX system calls.
3. **The Operating System & Userspace (100% Custom Rust):**
   - **Millelith Init (PID 1):** The very first process spawned by the kernel, responsible for mounting filesystems, reaping zombie processes, and managing system services.
   - **Millelith Shell:** A custom, interactive, POSIX-compliant command-line shell with syntax parsing, pipes, and job control.
   - **Millelith Core Utilities:** Re-engineering standard Unix utilities (`ls`, `cat`, `ps`, `kill`) in modern, memory-safe Rust.

---

## 2. System Architecture

```
+-------------------------------------------------------------------------+
|                        Millelith OS Architecture                         |
+-------------------------------------------------------------------------+
| [Layer 3: Userspace & Apps (100% Custom Rust)]                          |
|  - Millelith Init (PID 1): Mounts /proc, /sys, /dev, manages services   |
|  - Millelith Shell: Interactive CLI, AST parser, job control, pipelines |
|  - Millelith Coreutils: Essential Unix utilities in memory-safe Rust    |
|  - Graphical Environment: Framebuffer / Wayland terminal interface      |
+-------------------------------------------------------------------------+
                                    │ POSIX Syscalls (fork, exec, pipe, ioctl)
+-----------------------------------v-------------------------------------+
| [Layer 2: The Hardware Engine (The Linux Kernel - vmlinuz)]             |
|  - Universal Hardware Support: Wi-Fi 6, 3D GPU (DRM/KMS), NVMe, USB 3.0 |
|  - Virtual Memory (Paging), Hardware Ring 0 Protection, CPU Scheduler  |
|  - Network Stack: TCP/IP, WPA3, Socket layer                            |
+-------------------------------------------------------------------------+
                                    │ Linux 64-bit Boot Protocol / EFI Handover
+-----------------------------------v-------------------------------------+
| [Layer 1: Custom Rust UEFI Bootloader (crates/bootloader)]              |
|  - Zero-dependency bare-metal PE32+ executable (x86_64-unknown-uefi)    |
|  - Probes Graphics Output Protocol (GOP) for high-res splash screen     |
|  - Locates and loads /boot/vmlinuz and /boot/initrd from FAT32/EXT4     |
|  - Sets up boot parameters (command line, memory map) and transfers CPU |
+-------------------------------------------------------------------------+
```

---

## 3. Phased Implementation Roadmap

### Phase 1: Custom Rust UEFI Bootloader (`crates/bootloader`)
*Current Milestone: Establishing the firmware bootloader in pure zero-dependency Rust.*
- [x] Zero-dependency freestanding runtime (`#![no_std]`, `#[panic_handler]`, `EfiStatus`, `EfiHandle`).
- [x] Modular architecture (`src/uefi.rs` for protocols, `src/main.rs` for boot logic).
- [x] Verified text console output in QEMU (`Hello from Millelith OS!`).
- [x] Implement `EfiGuid`, `EfiBootServices`, and `LocateProtocol`.
- [x] Query Graphics Output Protocol (GOP) and obtain linear framebuffer pointer and stride.
- [x] Render Millelith Red splash screen.
- [x] Implement FAT32 file reading protocol to load `vmlinuz` and `initrd` into physical RAM.
- [x] Implement Linux 64-bit EFI boot protocol handover (`ExitBootServices` and kernel jump).

### Phase 2: Millelith Init (PID 1 System in Rust)
*The critical heart of the OS userspace.*
- [x] Implement freestanding Rust PID 1 binary (`/sbin/init`).
- [x] Mount fundamental virtual filesystems: `/proc` (procfs), `/sys` (sysfs), `/dev` (devtmpfs).
- [x] Configure standard I/O file descriptors (`stdin=0`, `stdout=1`, `stderr=2`).
- [x] Implement POSIX signal handling (`SIGCHLD`, `SIGINT`, `SIGTERM`).
- [x] Implement orphan process reaping (preventing zombie processes).
- [ ] Spawn the primary Millelith login / shell session.

### Phase 3: Millelith Shell (Interactive Unix Shell in Rust)
*Mastering parsing, process control, and IPC.*
- [ ] Interactive REPL with prompt rendering and line editing.
- [ ] Command string lexer and Abstract Syntax Tree (AST) parser.
- [ ] Process execution pipeline using `fork()` and `execve()`.
- [ ] Unix pipelines (`|`) using inter-process `pipe()` and `dup2()`.
- [ ] File redirection (`<`, `>`, `>>`).
- [ ] Built-in commands (`cd`, `exit`, `help`, `export`).

### Phase 4: Millelith Core Utilities (Rust Coreutils)
*Replacing GNU/BusyBox with memory-safe Rust implementations.*
- [ ] File manipulation: `ls`, `cat`, `cp`, `mv`, `rm`, `mkdir`.
- [ ] Process inspection: `ps` (parsing `/proc`), `kill`.
- [ ] System diagnostics: `uname`, `free`, `uptime`.

### Phase 5: Distribution Packaging & Physical PC Deployment
*Creating bootable media for physical computers.*
- [ ] Construct standalone root filesystem (`rootfs`) image.
- [ ] Generate bootable GPT USB disk image (`millelith.img`) containing `/EFI/BOOT/BOOTX64.EFI`, `/boot/vmlinuz`, and `/boot/initrd`.
- [ ] Boot Millelith OS on physical PC bare metal: verify Wi-Fi connectivity, GPU display, and interactive shell.

---

## 4. Computer Science Concepts Mastered

1. **Systems Architecture & Firmware:**
   - 64-bit Calling Conventions (`extern "efiapi"`), raw memory pointers, alignment padding.
   - Hardware framebuffer rendering (GOP), scanline strides, linear pixel packing.
2. **Operating System Fundamentals:**
   - POSIX system call interface (`fork`, `execve`, `mmap`, `pipe`, `ioctl`).
   - Process lifecycle, signals, and PID 1 orphan reaping.
   - Virtual filesystems (`/proc`, `/sys`, `/dev`).
3. **Compilers & Interpreters:**
   - Command grammar, lexing, parsing, and AST construction in the shell.
4. **Concurrency & Networking:**
   - Multi-process communication, file descriptors, Unix domain sockets, and network sockets.

