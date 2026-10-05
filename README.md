<p align="center">
  <img src="assets/branding/millelith-os-logo.png" alt="Millelith OS logo" width="720">
</p>

# Millelith OS

<p align="center">
  <a href="#progress"><img src="https://img.shields.io/badge/status-development-2563eb?style=flat-square" alt="Status: development"></a>
  <a href="#what-i-want-to-learn"><img src="https://img.shields.io/badge/language-C%2B%2B20-00599C?style=flat-square&amp;logo=cplusplus&amp;logoColor=white" alt="Primary language: C++20"></a>
  <a href="#the-plan"><img src="https://img.shields.io/badge/platform-amd64-2563eb?style=flat-square" alt="Initial platform: amd64"></a>
  <a href="#progress"><img src="https://img.shields.io/badge/boot-UEFI_app_verified-2ea44f?style=flat-square" alt="Boot: UEFI application verified"></a>
  <a href="https://github.com/kenzietann/millelithos/stargazers"><img src="https://img.shields.io/github/stars/kenzietann/millelithos?style=flat-square&amp;logo=github&amp;logoColor=white&amp;color=2563eb" alt="GitHub stars"></a>
</p>

I'm building Millelith OS to learn how computers work. I know basic C++ syntax,
but I want to understand what happens after the code is compiled: how the CPU
runs it, where its data goes, and how it talks to hardware.

The plan is to write my own bootloader and kernel, then build the rest of the
OS around them. I'm starting with x86-64, UEFI, and QEMU, using C++20 and
assembly where it's needed.

Right now, the C++ code boots as a UEFI application, prints text, and fills the
screen with a color. That's the starting point for the bootloader. There isn't
a separate kernel yet.

## The plan

The UEFI application in `boot/uefi/` will prepare the machine, load the kernel,
and pass along the information it needs to start. Once the kernel is ready to
take over, the bootloader will call `ExitBootServices` and transfer control.

Clang and LLD handle compilation and linking. QEMU gives me a place to test
changes, while UEFI provides the firmware services used during startup.

I also built a Debian live ISO along the way. That experiment helped me explore
the difference between assembling a Linux system and writing a kernel. Its
build setup has been removed now that I'm focusing on my own kernel.

## What I want to learn

I'm starting with types, pointers, structs, binary, and hexadecimal. I want to
be able to follow a piece of code through compilation and linking, then
understand the instructions, registers, calling conventions, and memory it
uses when it runs.

As the kernel grows, I'll work on memory allocation, interrupts, and drivers.
That will also give me reasons to learn data structures and algorithms:
choosing how to track free memory, queue work, look up data, and compare the
time and memory costs of different approaches.

Later, I want to understand processes, system calls, filesystems, and networking.
Permissions, input validation, and isolation will matter as soon as the system
starts handling programs and data of its own.

I'm taking this one step at a time. I want to be able to explain each part as
I build it, so I'm starting with small changes I can test and understand.

## Progress

### 1. UEFI startup

- [x] Boot an x86-64 EFI application in QEMU.
- [x] Print messages through the firmware console.
- [x] Keep the application running after startup.

### 2. Drawing to the screen

- [x] Find the Graphics Output Protocol and read its display information.
- [x] Check pointers, buffer size, and pixel formats before writing.
- [x] Fill the visible screen with RGB `(24, 65, 110)`, or `#18416E`.

The startup messages appear first, then the framebuffer fill covers them.
The final screen is dark blue.

### 3. The Linux experiment

- [x] Set up a Debian 13 live-build configuration.
- [x] Build a live ISO in an AMD64 Debian container.
- [x] Boot it through UEFI in QEMU and reach a terminal.

This worked on 1 October 2026. The image ran Debian 13.7 with kernel
`6.12.111+deb13-amd64` on `x86_64`. This was a separate experiment from the
kernel work; its build files have since been removed.

### 4. Reading the UEFI memory map

This is the next step. Before managing memory, I need to understand how firmware
describes it and which regions are available.

- [ ] Understand addresses, regions, pages, and memory ownership.
- [ ] Define the descriptor and `GetMemoryMap` interface.
- [ ] Allocate a buffer and read the map, handling the required buffer size.
- [ ] Check the descriptor format and inspect the regions.
- [ ] Understand how the map key is used by `ExitBootServices`.

For this step, the application will stay in the UEFI boot services environment.

### 5. Starting my own kernel

- [ ] Create a separate freestanding C++ kernel executable.
- [ ] Learn its executable format and load it into memory.
- [ ] Pass startup data, including the framebuffer and memory map.
- [ ] Get the final memory map, call `ExitBootServices`, and enter the kernel.
- [ ] Show output from the kernel without using firmware boot services.

After that, I'll work on kernel startup, the stack, logging, and CPU exceptions.
Then come physical memory allocation, page tables, a heap, timer interrupts,
and keyboard input.

Further down the line are tasks and scheduling, protected user programs,
system calls, device discovery, storage, and a filesystem. Networking, a shell,
and a graphical interface will follow as the basics become usable. Native
Millelith tools and testing on physical hardware are goals for later in the project.

## Files

The current UEFI application is in `boot/uefi/main.cpp`. Its UEFI types,
structures, and function signatures are in `boot/uefi/uefi.hpp`. A kernel
directory will be added when I start the separate executable.

Artwork goes under `assets/`, with the logo and icon in `assets/branding/`.
Compiled binaries, generated images, `esp/`, and writable firmware state
are ignored by Git.

## Artwork

<img src="assets/branding/millelith-os-icon.png" alt="Millelith OS icon" width="96">

- [Icon](assets/branding/millelith-os-icon.png)
- [Logo](assets/branding/millelith-os-logo.png)

## Running the UEFI application

These are the commands for the current console and framebuffer code.

### Requirements on macOS

You'll need Clang with the `x86_64-unknown-uefi` target, plus LLD and QEMU.
The Homebrew dependencies are:

```sh
brew install lld qemu
```

### Compile

Run this from the repository root:

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

UEFI uses `EFI/BOOT/BOOTX64.EFI` as the default boot path on x86-64 media.

Create the writable firmware state if you haven't already:

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

You should see the startup messages followed by a dark blue screen. The
application then stays in its `hlt` loop.

After editing the C++ code, compile it again and copy the new `BOOTX64.EFI`
into `esp/EFI/BOOT/` before restarting QEMU.

## How I'm working

I type and test each step, with explanations as I go. If I can't explain what
a piece of code does, I go back to the concept behind it before moving on.
Keeping code, notes, and documentation open is part of that process.

I record compilation, boot, and feature tests separately. A successful build
doesn't tell me whether the program boots, and a successful boot doesn't tell
me whether every feature works.

## References

- [UEFI Boot Services and memory allocation](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html)
- [UEFI status codes](https://uefi.org/specs/UEFI/2.10/Apx_D_Status_Codes.html)