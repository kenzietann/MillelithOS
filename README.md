# Millelith OS

Millelith OS is an operating system project based on the Linux kernel for
desktop use, software development, and Millelith Security's internal tools
for authorized security assessments.

The initial target is AMD64/x86-64 computers with UEFI. The selected
distribution base is Debian 13 (`trixie`).

This repository currently contains a C++ UEFI boot experiment. The console
and framebuffer milestones were successfully tested in QEMU during earlier
learning stages. Linux distribution configuration and ISO images will be
recreated step by step.

## Project direction

Millelith OS aims to provide:

- A graphical desktop, terminal, file manager, networking, and system updates.
- Storage that persists across reboots.
- An installer for installing the system on a PC's disk.
- C/C++, Python, JavaScript, Go, and Rust development tools.
- Linux packages and containers for developer workflows.
- Packages for Millelith's internal tools.
- Engagement scope management, activity logging, and evidence storage
  for security work.

These are development goals. The Linux features have not yet been implemented
in a runnable image from the current repository.

The Linux kernel provides the foundation for process management, memory,
filesystems, networking, and drivers. The C++ experiment in `boot/uefi/`
is used to learn about booting and communicating with firmware.

## Milestone status

### Milestone 1 — UEFI boot and text output

- [x] Create an x86-64 EFI application.
- [x] Boot the application through UEFI firmware in QEMU.
- [x] Call the UEFI console and print messages.
- [x] Keep the application active after startup.

### Milestone 2 — Graphics and framebuffer

- [x] Locate the Graphics Output Protocol (GOP).
- [x] Read the display mode information and framebuffer address.
- [x] Validate pointers, buffer size, and supported pixel formats.
- [x] Write a color to every visible pixel position.
- [x] Display a dark blue screen using RGB `(24, 65, 110)` or `#18416E`.

Startup text is printed before the framebuffer is filled. Filling the screen
overwrites that text, so the final display is solid dark blue.

### Milestone 3 — First Linux boot

- [x] Select Debian 13 (`trixie`) as the base.
- [x] Set up tools on the Mac: Colima, Docker, and additional Lima guest agents.
- [ ] Verify that the Linux AMD64 build environment and Docker are usable.
- [ ] Create a minimal Debian live-build configuration.
- [ ] Produce a live ISO.
- [ ] Boot the ISO through UEFI in QEMU and reach a Linux terminal.

During the 1 October 2026 check, the `millelith-amd64` Colima profile was listed
as running, but its Docker connection was unavailable. The build environment
needs to be checked before ISO creation continues.

### Upcoming milestones

- [ ] Add a desktop and verify user interaction.
- [ ] Add development tools and test program compilation and execution.
- [ ] Verify networking, file access, and package updates.
- [ ] Add an installer and persistent storage.
- [ ] Verify that the system boots and preserves data after removing the ISO.
- [ ] Add Millelith OS visual branding.
- [ ] Package and test Millelith's internal tools.
- [ ] Add engagement policies, logging, and evidence storage.
- [ ] Test physical hardware and expand platform support.

## Current repository structure

```text
.
├── README.md
├── .gitignore
└── boot/
    └── uefi/
        ├── main.cpp
        └── uefi.hpp
```

- `uefi.hpp`: types, structures, and function signatures for UEFI interfaces.
- `main.cpp`: application entry point, text output, GOP lookup, and
  framebuffer filling.

Compiled EFI files, the generated EFI filesystem for QEMU, and writable
firmware state are excluded from Git through `.gitignore`.

## Running the UEFI experiment

The following commands run the C++ experiment currently in this repository.

### macOS requirements

- Clang with support for the `x86_64-unknown-uefi` target.
- LLD and QEMU.

Homebrew dependencies:

```sh
brew install lld qemu
```

### Compile

Run from the repository root:

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

### Prepare the EFI filesystem

```sh
mkdir -p esp/EFI/BOOT
cp BOOTX64.EFI esp/EFI/BOOT/BOOTX64.EFI
```

The path `EFI/BOOT/BOOTX64.EFI` is used for default boot on x86-64 UEFI media.

Create a copy of the writable firmware state if it does not exist:

```sh
if [ ! -f OVMF_VARS.fd ]; then
  cp "$(brew --prefix qemu)/share/qemu/edk2-i386-vars.fd" OVMF_VARS.fd
fi
```

### Run in QEMU

```sh
qemu-system-x86_64 \
  -machine q35,accel=tcg \
  -m 256M \
  -drive if=pflash,format=raw,unit=0,readonly=on,file="$(brew --prefix qemu)/share/qemu/edk2-x86_64-code.fd" \
  -drive if=pflash,format=raw,unit=1,file=OVMF_VARS.fd \
  -drive format=raw,file=fat:rw:esp \
  -net none
```

Expected result: the program prints startup messages and then fills the screen
with dark blue. The program remains active through its `hlt` loop.

After changing the C++ code, compile again and copy `BOOTX64.EFI` into
`esp/EFI/BOOT/` before restarting QEMU.

## Linux build environment

On an ARM64 Mac, Colima manages a virtual machine through Lima with QEMU as
the backend for running Linux AMD64. Once the Docker service is accessible,
Docker will run a Debian environment containing the ISO build tools.

Planned build flow:

```text
Mac → Colima (Linux VM) → Docker (build tools) → Millelith OS ISO
```

The final output is an ISO file that can be written to a USB drive to boot a PC.
Docker is used on the development machine; the PC does not require Docker
to boot Millelith OS.

The first ISO will be a live system that runs from the boot media. Installation
to a disk is a later milestone.

ISO build configuration and commands will be documented after they are created
and tested. There is currently no distribution configuration or Linux ISO
in this repository.

## Learning and development workflow

Each stage is completed in small steps: understand its purpose, type the code
or configuration, inspect the result, and test its behavior in QEMU.

Successful compilation, successful boot, and working features are recorded
as separate outcomes. Checklists are updated based on test results.

## References

- [Debian Live Manual](https://live-team.pages.debian.net/live-manual/html/live-manual.en.html)
- [UEFI status codes](https://uefi.org/specs/UEFI/2.10/Apx_D_Status_Codes.html)
