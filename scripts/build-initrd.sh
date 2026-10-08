#!/usr/bin/env bash
# Build all userspace binaries, install them into build/rootfs, and pack esp/initrd
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

ROOTFS="build/rootfs"
mkdir -p "$ROOTFS"/{bin,sbin,dev,proc,sys} esp

# Utilities from crates/coreutils, add new tool names here
COREUTILS=(cat echo)

# init and coreutils are freestanding (no_std, no libc), shell links against musl
cargo build --release -p init --target x86_64-unknown-none
# Static (non-PIE) layout: the kernel does not apply relocations to a bare no_std ELF,
# so a PIE build leaves its GOT zeroed and the first call through it jumps to address 0
RUSTFLAGS="-C relocation-model=static" cargo build --release -p coreutils --target x86_64-unknown-none
cargo build --release -p shell --target x86_64-unknown-linux-musl

cp target/x86_64-unknown-none/release/init "$ROOTFS/sbin/init"
cp target/x86_64-unknown-linux-musl/release/msh "$ROOTFS/bin/msh"
for tool in "${COREUTILS[@]}"; do
  cp "target/x86_64-unknown-none/release/$tool" "$ROOTFS/bin/$tool"
done
ln -sfn sbin/init "$ROOTFS/init"

# Pack the rootfs as an uncompressed newc cpio archive
(cd "$ROOTFS" && find . | cpio -o -H newc 2>/dev/null) > esp/initrd

echo "[OK] esp/initrd rebuilt ($(wc -c < esp/initrd | tr -d ' ') bytes)"
