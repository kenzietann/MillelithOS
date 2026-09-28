# From-Scratch C++ OS

An experimental operating system written in freestanding C++.

The project currently boots as a native x86-64 UEFI application, communicates with the firmware directly, and does not use a third-party bootloader or operating-system framework.

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