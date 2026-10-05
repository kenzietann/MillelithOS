<p align="center">
  <img src="assets/branding/millelith-os-logo.png" alt="Millelith OS logo" width="720">
</p>

# Millelith OS

<p align="center">
  <a href="#progress"><img src="https://img.shields.io/badge/status-development-2563eb?style=flat-square" alt="Status: development"></a>
  <a href="#what-i-want-to-learn"><img src="https://img.shields.io/badge/language-Rust-DEA584?style=flat-square&amp;logo=rust&amp;logoColor=white" alt="Primary language: Rust"></a>
  <a href="#the-plan"><img src="https://img.shields.io/badge/platform-amd64-2563eb?style=flat-square" alt="Initial platform: amd64"></a>
  <a href="#the-plan"><img src="https://img.shields.io/badge/boot-UEFI_native-2ea44f?style=flat-square" alt="Boot: UEFI native"></a>
  <a href="#the-plan"><img src="https://img.shields.io/badge/target-PC_ready-f97316?style=flat-square" alt="Target: PC ready"></a>
  <a href="https://github.com/kenzietann/millelithos/stargazers"><img src="https://img.shields.io/github/stars/kenzietann/millelithos?style=flat-square&amp;logo=github&amp;logoColor=white&amp;color=2563eb" alt="GitHub stars"></a>
</p>

I'm building Millelith OS to learn how computers work from the ground up. I want to understand what happens after code compiles: how the CPU executes instructions, how physical memory is mapped and protected, how hardware interrupts fire, and how an operating system coordinates programs.

I started with a C++ UEFI application proof-of-concept that booted in QEMU, initialized the GOP framebuffer, and filled the screen with `#18416E` blue (now safely preserved in the [`archive/cpp-uefi`](https://github.com/kenzietann/millelithos/tree/archive/cpp-uefi) branch).

Now, I'm building the real system in **100% bare-metal Rust** (`#![no_std]`). The project is engineered to be **PC-ready from Day 1**—designed not just for emulators, but to boot and run on physical x86-64 PC hardware without relying on throwaway legacy shortcuts.

For the complete technical specifications and phased milestone roadmap, see [PRD.md](PRD.md).

---

## The Architecture

Millelith OS avoids black-box bootloaders. It exercises full-pipeline control across three dedicated Rust crates:

```
crates/
├── boot_info/     # Shared data contracts (framebuffer info, memory map, ACPI pointers)
├── bootloader/    # Custom Rust UEFI application (target: x86_64-unknown-uefi)
└── kernel/        # Freestanding higher-half OS kernel (target: x86_64-unknown-none)
```

### 1. Bootloader (`crates/bootloader`)
- Runs as an EFI binary (`/EFI/BOOT/BOOTX64.EFI`) on GPT/FAT32 media.
- Sets the native GOP display mode and queries the linear framebuffer.
- Loads and parses the 64-bit kernel ELF binary into physical RAM.
- Discovers the ACPI RSDP pointer from UEFI configuration tables.
- Prepares initial 4-level page tables with higher-half kernel mappings (`0xFFFF_8000_0000_0000`).
- Retrieves the final UEFI memory map, exits boot services, and transfers control to the kernel.

### 2. Kernel (`crates/kernel`)
- Freestanding `#![no_std]` Rust binary starting at `_start(boot_info: &'static BootInfo) -> !`.
- **Display & Diagnostics:** Framebuffer console with an embedded bitmap font engine and a dedicated on-screen Kernel Panic screen (BSOD) for physical hardware crash reporting.
- **Hardware & Interrupts:** GDT, TSS with double-fault stack, and IDT for CPU exceptions. Legacy 8259 PIC is permanently masked in favor of the **Local APIC** and **APIC Timer**.
- **Memory Subsystem:** Physical frame allocator strictly filtering `EfiConventionalMemory` (protecting ACPI, NVS, and MMIO zones), active 4-level paging, and a dynamic heap allocator.
- **Concurrency:** Cooperative async/await task execution graduating into a preemptive round-robin scheduler.

---

## What I Want to Learn

- **Hardware Abstraction & Firmware:** UEFI services, Graphics Output Protocol (GOP), ACPI tables (`RSDP`, `MADT`), and the Local APIC.
- **Memory Management:** Physical frame allocators (bitmap/buddy), x86-64 4-level virtual page tables, TLB invalidation, and heap allocators (bump, linked-list, slab).
- **CPU Architecture & Traps:** Privilege rings (Ring 0 vs. Ring 3), descriptor tables (GDT, IDT, TSS), exception handlers, and hardware context switching.
- **Concurrency & Operating System Design:** Async executors, preemptive timer-driven schedulers, virtual filesystems (VFS), and system calls (`syscall`/`sysret`).

---

## Progress

### 1. Proof of Concept (C++ UEFI)
- [x] Boot an x86-64 EFI application in QEMU.
- [x] Print text via UEFI firmware console services.
- [x] Acquire the Graphics Output Protocol (GOP) linear framebuffer.
- [x] Verify pixel formats and fill the screen with `#18416E` blue.
- [x] Query UEFI memory map descriptor sizes.
- [x] *Archived to branch [`archive/cpp-uefi`](https://github.com/kenzietann/millelithos/tree/archive/cpp-uefi).*

### 2. Project Redesign (Rust PC-Native)
- [x] Created comprehensive Product Requirements Document ([PRD.md](PRD.md)).
- [x] Established "PC-Ready from Day 1" architecture (GOP, Local APIC, higher-half paging).
- [ ] Initialize Cargo workspace (`boot_info`, `bootloader`, `kernel`).
- [ ] Set up disk image generation script for QEMU and physical USB drives.

### 3. The Rust UEFI Bootloader
- [ ] Implement UEFI GOP query and framebuffer acquisition via `uefi-rs`.
- [ ] Read and parse `kernel.elf` from the boot media filesystem.
- [ ] Locate the ACPI RSDP table pointer.
- [ ] Query UEFI memory map, allocate kernel stack, and exit boot services.
- [ ] Jump into kernel `_start` with `BootInfo`.

### 4. Framebuffer Console & CPU Traps
- [ ] Embedded 8x16 bitmap font renderer and `println!` screen console.
- [ ] Kernel panic screen displaying registers (`RIP`, `RSP`, `CR2`) on crash.
- [ ] GDT & TSS setup with a dedicated double-fault interrupt stack.
- [ ] IDT exception handlers for breakpoints, page faults, and general protection faults.

### 5. Memory Management & Modern Interrupts
- [ ] Physical frame allocator parsing the UEFI memory map.
- [ ] Active 4-level paging manager (higher-half mapping).
- [ ] Kernel heap allocator enabling Rust's `alloc` crate (`Vec`, `String`, `Box`).
- [ ] Mask legacy 8259 PIC; configure Local APIC and APIC Timer via ACPI.

---

## Artwork

<img src="assets/branding/millelith-os-icon.png" alt="Millelith OS icon" width="96">

- [Icon](assets/branding/millelith-os-icon.png)
- [Logo](assets/branding/millelith-os-logo.png)

---

## Toolchain & Requirements

### Host Dependencies
- **Rust Nightly:** `rustup default nightly`
- **Rust Targets:**
  ```sh
  rustup target add x86_64-unknown-uefi
  rustup target add x86_64-unknown-none
  rustup component add rust-src llvm-tools-preview
  ```
- **QEMU:** `brew install qemu` (macOS) or `sudo apt install qemu-system-x86` (Linux)

---

## References

- [Philipp Oppermann's Writing an OS in Rust](https://os.phil-opp.com/)
- [UEFI Specification 2.10](https://uefi.org/specifications)
- [OSDev Wiki: UEFI](https://wiki.osdev.org/UEFI)
- [OSDev Wiki: APIC](https://wiki.osdev.org/APIC)

