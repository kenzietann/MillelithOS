# Product Requirements Document (PRD): Millelith OS

**Project Name:** Millelith OS  
**Status:** Inception & Architectural Migration  
**Author & Lead Developer:** Kenzie Tann  
**Implementation Language:** 100% Rust (`#![no_std]`, bare-metal `x86_64`)  
**Core Purpose:** An operating system built from absolute scratch to learn and master Computer Science fundamentals in a single unified project (firmware protocols, hardware registers, memory paging, allocators, concurrency, filesystems, and user mode), engineered to boot and run on physical PC hardware.

---

## 1. Executive Summary & Vision

### 1.1 Vision Statement
Millelith OS is a bare-metal, educational operating system engineered entirely in Rust for `x86_64`. Rather than relying on black-box bootloaders or emulator-only shortcuts, Millelith OS exercises **full-pipeline ownership**: from firmware handoff in a custom Rust UEFI bootloader to an independent long-mode Rust kernel running on real, physical PC hardware.

### 1.2 Core Architectural Principles
1. **Full-Pipeline Control (No Black Boxes):**
   - The bootloader is written in Rust using UEFI primitives (`uefi-rs`), directly managing firmware services, acquiring the memory map, setting up the GOP display, loading the kernel ELF, exiting boot services, and jumping into the kernel.
2. **Real Hardware First:**
   - Avoid emulator-only shortcuts (e.g., assuming legacy VGA text mode `0xb8000` exists or assuming COM1 serial ports are physically present). The OS must run reliably on physical x86_64 motherboards.
3. **Modern Graphical Output:**
   - Standardize on the **UEFI Graphics Output Protocol (GOP)** linear framebuffer. Text rendering is performed via an embedded bitmap font engine with a robust kernel panic screen.
4. **Rust Memory & Concurrency Safety:**
   - Bare-metal code enforces zero-cost abstractions, strict ownership boundaries, and isolated `unsafe` blocks with explicit safety contracts.
5. **Pedagogical Progression:**
   - Build concepts incrementally: starting with cooperative async/await multitasking and graduating into a preemptive timer-driven scheduler with context switching.

---

## 2. System Architecture

```
+-------------------------------------------------------------------------+
|                          Millelith OS Architecture                       |
+-------------------------------------------------------------------------+
| [Stage 1: Custom Rust Bootloader]                                       |
|  - Target: x86_64-unknown-uefi (uefi-rs)                                |
|  - Boot Media: GPT Partitioned USB / FAT32 EFI System Partition (ESP)   |
|  - Path: /EFI/BOOT/BOOTX64.EFI                                          |
|  - Probes GOP framebuffer & queries optimal resolution                  |
|  - Reads & parses kernel ELF executable from boot media                 |
|  - Queries UEFI memory map & strictly filters EfiConventionalMemory     |
|  - Locates ACPI RSDP pointer from UEFI configuration tables             |
|  - Sets up initial 4-level page tables (identity & higher-half mapping) |
|  - Exits Boot Services (ExitBootServices)                               |
|  - Passes BootInfo (framebuffer, memory map, ACPI) to Kernel Entry      |
+-------------------------------------------------------------------------+
                                    |
                                    v
+-------------------------------------------------------------------------+
| [Stage 2: Millelith Kernel]                                             |
|  - Target: x86_64-unknown-none                                          |
|  - Entry Point: _start(boot_info: &'static BootInfo) -> !               |
|                                                                         |
|  +-------------------------------------------------------------------+  |
|  | Hardware Abstraction Layer & Interrupts                           |  |
|  | - GDT & Task State Segment (Double Fault Stack)                   |  |
|  | - IDT: CPU Exception Handlers (Page Faults, GPF, Breakpoints)     |  |
|  | - ACPI Parser (RSDP -> XSDT -> MADT)                             |  |
|  | - Interrupt Controller: 8259 PIC (initial) -> Local APIC / IOAPIC |  |
|  | - Timers: PIT -> Local APIC Timer / HPET                          |  |
|  | - Input: PS/2 Controller (Legacy USB Emulation) -> xHCI (Future)  |  |
|  | - Diagnostics: Linear GOP Framebuffer Console + Serial UART 16550 |  |
|  +-------------------------------------------------------------------+  |
|                                    |                                    |
|  +-------------------------------------------------------------------+  |
|  | Memory Management Subsystem                                       |  |
|  | - Physical Frame Allocator (Bitmap / Buddy Allocator)             |  |
|  | - Virtual Memory Paging (Active 4-Level Page Table management)   |  |
|  | - Higher-Half Kernel Mapping & MMIO reservation                   |  |
|  | - Dynamic Kernel Heap (Bump -> Linked-List -> Slab Allocator)     |  |
|  +-------------------------------------------------------------------+  |
|                                    |                                    |
|  +-------------------------------------------------------------------+  |
|  | Concurrency & Scheduling                                          |  |
|  | - Level A: Cooperative async/await executor with Wakers           |  |
|  | - Level B: Preemptive timer-based round-robin thread scheduler    |  |
|  | - SMP (Symmetric Multiprocessing) foundation via APIC            |  |
|  +-------------------------------------------------------------------+  |
|                                    |                                    |
|  +-------------------------------------------------------------------+  |
|  | Storage & Filesystem                                              |  |
|  | - Virtual File System (VFS) abstraction traits                    |  |
|  | - In-memory RamFS / InitRD (tarfs)                                |  |
|  +-------------------------------------------------------------------+  |
|                                    |                                    |
|  +-------------------------------------------------------------------+  |
|  | User Space & Isolation                                            |  |
|  | - Ring 0 -> Ring 3 privilege transitions (syscall / sysret)      |  |
|  | - Isolated per-process page tables                                |  |
|  | - Interactive Shell / CLI                                         |  |
|  +-------------------------------------------------------------------+  |
+-------------------------------------------------------------------------+
```

---

## 3. Phased Implementation Roadmap

### Phase 0: Workspace Reorganization & Repository Split
- [ ] Create git branch `archive/cxx-uefi` to preserve the complete C++ codebase.
- [ ] Prepare clean `main` branch with a Cargo multi-crate workspace:
  - `crates/bootloader`: Rust UEFI application (`x86_64-unknown-uefi`).
  - `crates/kernel`: Freestanding OS kernel (`x86_64-unknown-none`).
  - `crates/boot_info`: Shared struct definitions between bootloader and kernel.
- [ ] Implement an automated image generator script to build a bootable FAT32 ESP disk image.

### Phase 1: The Rust UEFI Bootloader
- [ ] Initialize `bootloader` crate using `uefi-rs`.
- [ ] Query and initialize UEFI Graphics Output Protocol (GOP) to acquire framebuffer info.
- [ ] Locate and load the kernel ELF from the FAT32 ESP filesystem into memory.
- [ ] Locate the ACPI RSDP pointer from UEFI system configuration tables and add to `BootInfo`.
- [ ] Retrieve memory map descriptors and calculate physical memory layout.
- [ ] Call `exit_boot_services`, construct `BootInfo`, and jump to the kernel entry point.

### Phase 2: Framebuffer Console & Hardware Diagnostics
- [ ] Implement embedded 8x16 bitmap font renderer in the kernel.
- [ ] Build a thread-safe `ConsoleWriter` supporting scrolling, word wrapping, and ANSI color codes.
- [ ] Expose `print!` and `println!` macros writing to the screen.
- [ ] Implement a **Kernel Panic Screen**: Display a blue/red screen with CPU registers, faulting address (`CR2`), and stack trace directly on the framebuffer when a crash occurs on physical hardware.
- [ ] Configure Serial UART 16550 driver with `serial_print!` and `serial_println!` macros for QEMU host logging.

### Phase 3: Hardware Initialization & Interrupts
- [ ] Global Descriptor Table (GDT) and Task State Segment (TSS) with dedicated double-fault stack.
- [ ] Interrupt Descriptor Table (IDT) for all x86 exceptions (Page Fault, Double Fault, GPF).
- [ ] Programmable Interrupt Controller (8259 PIC) setup and interrupt masking.
- [ ] PS/2 keyboard driver handling scancodes and modifiers (relying on BIOS USB legacy emulation on real hardware).
- [ ] PIT (Programmable Interval Timer) generating periodic clock interrupts.

### Phase 4: Physical, Virtual, and Dynamic Memory
- [ ] **Physical Frame Allocator:**
  - Parse bootloader memory map to construct a frame allocator tracking free physical 4 KiB frames.
  - Strictly isolate `EfiConventionalMemory` from ACPI, NVS, and MMIO reserved zones.
- [ ] **Page Tables:**
  - Implement mapping and unmapping functions for x86_64 4-level paging.
  - Ensure kernel runs in the higher half (`0xFFFF_8000_0000_0000` or offset direct mapping).
- [ ] **Kernel Heap:**
  - Milestone 4a: Simple Bump Allocator to enable `alloc` crate.
  - Milestone 4b: Linked-List Allocator tracking freed chunks.
  - Milestone 4c: Fixed-size block (Slab) allocator for fast, low-fragmentation small allocations.

### Phase 5: Concurrency & Scheduling
- [ ] **Cooperative Concurrency:**
  - Implement `Task` struct and `Executor` for Rust `Future`s.
  - Create async keyboard event queue awakened by hardware interrupt wakers.
- [ ] **Preemptive Multitasking:**
  - Define Thread Control Block (TCB) capturing registers (`rsp`, `rip`, general registers, `rflags`).
  - Implement context switch routine in assembly triggered by timer interrupt.
  - Round-robin scheduler managing multiple execution threads.

### Phase 6: Real Hardware Subsystems (ACPI & APIC)
- [ ] Parse ACPI RSDP -> XSDT -> MADT tables.
- [ ] Disable legacy 8259 PIC and enable x86 Local APIC & IOAPIC.
- [ ] Calibrate and switch to Local APIC Timer / HPET for microsecond-precision scheduling.

### Phase 7: Filesystem (VFS) & User Mode
- [ ] VFS traits (`Node`, `File`, `Directory`).
- [ ] In-memory initial RAM disk (InitRD / tarfs) loaded by the bootloader.
- [ ] Separation of Ring 0 and Ring 3 segments.
- [ ] `syscall` / `sysret` instruction setup (MSR registers).
- [ ] User-space process loading and execution.
- [ ] Interactive text shell running in user mode.

---

## 4. Real-Hardware & Physical PC Requirements

To ensure Millelith OS runs on physical PC motherboards and not just QEMU:

### 4.1 Boot Media & UEFI Firmware Standards
- **Partitioning:** GPT (GUID Partition Table) with a dedicated **FAT32 EFI System Partition (ESP)** (Partition Type GUID: `c12a7328-f81f-11d2-ba4b-00a0c93ec93b`).
- **Binary Placement:** Bootloader must be named and placed at `/EFI/BOOT/BOOTX64.EFI`.
- **Firmware Support:** UEFI 2.x standard 64-bit firmware (Class 2 or pure Class 3 without CSM).
- **Secure Boot:** Must be **Disabled** in motherboard firmware settings (unsigned third-party binaries cannot boot with active Microsoft UEFI CA keys).

### 4.2 Hardware Divergence Matrix: QEMU vs. Physical PC

| Subsystem | QEMU Behavior | Physical PC Reality | Millelith OS Architecture Requirement |
| :--- | :--- | :--- | :--- |
| **Display** | Emulates legacy VGA text mode (`0xb8000`) | Often non-existent on modern UEFI PCs | **GOP linear framebuffer** with software font engine. No reliance on legacy VGA memory. |
| **Console Output** | Virtual COM1 serial port (`0x3F8`) logs to terminal | No physical serial port wired to CPU | **On-screen panic screen** and framebuffer logging are mandatory for hardware debugging. |
| **Interrupts** | Emulates 8259 PIC & 8254 PIT | Dual PIC is legacy; APIC/IOAPIC standard | ACPI table parsing (`MADT`) to enable **Local APIC** and disable legacy PIC. |
| **Timers** | PIT ticks reliably at 1.193182 MHz | PIT may drift or be emulated poorly | **Local APIC Timer** calibrated via TSC (Time Stamp Counter) or HPET. |
| **Keyboard** | Virtual PS/2 controller at ports `0x60`/`0x64` | USB keyboard hardware | Require motherboard BIOS **"USB Legacy Support"**; roadmap towards native xHCI. |
| **Memory Map** | Contiguous RAM blocks | Fragmented by MMIO, ACPI tables, NVS, UEFI runtime | Strict filtering: **Only allocate from `EfiConventionalMemory`**. Respect all reserved firmware ranges. |

### 4.3 Physical Deployment Workflow (Flashing to USB)
1. Format a physical USB flash drive with a GPT partition scheme and a FAT32 partition labeled `MILLELITH`.
2. Copy compiled artifacts:
   - `crates/bootloader/target/x86_64-unknown-uefi/release/bootloader.efi` $\to$ `USB:/EFI/BOOT/BOOTX64.EFI`
   - `crates/kernel/target/x86_64-unknown-none/release/kernel.elf` $\to$ `USB:/kernel.elf`
3. Insert USB into target PC, enter BIOS boot menu (`F11`/`F12`), select the UEFI USB drive, and boot directly on bare metal.

---

## 5. Verification & Testing Strategy

1. **Host-Side Testing (Dual Testing Pipeline):**
   - **QEMU Fast-Loop:** Automated test runner with `-device isa-debug-exit` and serial console output for rapid continuous development.
   - **QEMU Bare-Metal Parity Mode:** Launch QEMU with `-bios /usr/share/OVMF/OVMF_CODE.fd` and modern parameters disabling legacy VGA (`-vga none -device virtio-gpu-pci`) to mirror physical UEFI hardware.
2. **Crash Diagnostics on Real Silicon:**
   - Exception handlers intercepting faults dump full state to the GOP framebuffer:
     - Fault type & exception error code.
     - General-purpose registers (`rax`, `rbx`, `rcx`, `rdx`, `rsi`, `rdi`, `rsp`, `rbp`, `r8-r15`).
     - Control registers (`cr0`, `cr2` fault address, `cr3` page directory base, `cr4`).
     - CPU flags (`rflags`) and instruction pointer (`rip`).
3. **Safety Contracts:**
   - Kernel strictly isolates raw hardware pointer dereferences, I/O port writes, and MSR manipulation behind safe abstractions.
