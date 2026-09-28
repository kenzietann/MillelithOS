# Millelith OS

An experimental operating system written in freestanding C++.

The project currently boots as a native x86-64 UEFI application, communicates with the firmware directly, and does not use a third-party bootloader or operating-system framework.

## Vision

Millelith OS is a standalone operating system built from scratch for Millelith Security operators.

It provides the normal foundations of a modern operating system, including protected processes, virtual memory, filesystems, hardware drivers, networking, a terminal, and a graphical desktop.

Millelith OS runs only trusted Millelith applications. Security tools execute as isolated user processes and must follow the active authorized-engagement scope. The operating system enforces target restrictions, permissions, rate limits, audit logging, and evidence preservation below the application layer.

Millelith OS does not aim to run Windows, macOS, or Linux applications directly. Applications are compiled specifically for the Millelith platform.

## Version 1

The first usable version will:

- Boot into an interactive Millelith shell
- Load an engagement scope from disk
- Support one virtual network adapter
- Configure IPv4 networking
- Run one built-in native assessment tool
- Reject targets outside the loaded scope
- Record commands and results
- Save evidence to disk
- Provide no mechanism for executing arbitrary third-party programs

## Current milestone

The first UEFI application successfully:

- Boots on x86-64 UEFI firmware
- Runs under QEMU with TianoCore firmware
- Calls the UEFI console directly
- Prints `Hello from OS!`
- Remains active after startup

## Project structure

```text
boot/
└── uefi/
    └── main.cpp
```

Generated EFI executables and virtual firmware state are excluded from Git.

## Requirements

On macOS:

```sh
brew install lld qemu
```

The compiler must support the `x86_64-unknown-uefi` target.

## Build

```sh
clang++ \
  --target=x86_64-unknown-uefi \
  -std=c++20 \
  -ffreestanding \
  -fno-exceptions \
  -fno-rtti \
  -fno-stack-protector \
  -nostdlib \
  boot/uefi/main.cpp \
  -o BOOTX64.EFI
```

## Prepare the EFI filesystem

```sh
mkdir -p esp/EFI/BOOT
cp BOOTX64.EFI esp/EFI/BOOT/BOOTX64.EFI
cp "$(brew --prefix qemu)/share/qemu/edk2-i386-vars.fd" OVMF_VARS.fd
```

UEFI searches for the default x86-64 boot program at:

```text
EFI/BOOT/BOOTX64.EFI
```

## Run

```sh
qemu-system-x86_64 \
  -machine q35,accel=tcg \
  -m 256M \
  -drive if=pflash,format=raw,unit=0,readonly=on,file="$(brew --prefix qemu)/share/qemu/edk2-x86_64-code.fd" \
  -drive if=pflash,format=raw,unit=1,file=OVMF_VARS.fd \
  -drive format=raw,file=fat:rw:esp \
  -net none
```

Expected output:

```text
Hello from OS!
```

## Roadmap

- [x] Build a native x86-64 EFI application
- [x] Print through the UEFI text console
- [ ] Access the UEFI graphics framebuffer
- [ ] Obtain the firmware memory map
- [ ] Load a separate kernel executable
- [ ] Call `ExitBootServices`
- [ ] Transfer control to the kernel
- [ ] Add physical memory management
- [ ] Add interrupts and exception handling
- [ ] Add keyboard, storage, display, and PCI drivers
- [ ] Add support for additional architectures

## Design goals

- Freestanding C++
- Clear separation between firmware, architecture, kernel, and drivers
- Minimal external dependencies
- Support for a broad range of computers over time
- Testable under emulators before physical hardware
