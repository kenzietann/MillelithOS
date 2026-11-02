#!/usr/bin/env bash
# Build all userspace binaries, install them into build/rootfs, and pack esp/initrd
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

ROOTFS="build/rootfs"
mkdir -p "$ROOTFS"/{bin,sbin,dev,proc,sys} esp

# Utilities from crates/coreutils, add new tool names here
COREUTILS=(cat echo ls mkdir cp rm touch mv rmdir stat grep more less sed zcat)

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

# gzip samples for testing zcat: one without a stored file name, one with
mkdir -p "$ROOTFS/samples"
printf 'hello from gzip\n' | gzip -9 > "$ROOTFS/samples/hello.gz"
printf 'named member\n' > "$ROOTFS/samples/named.txt"
gzip -9 -c "$ROOTFS/samples/named.txt" > "$ROOTFS/samples/named.gz"
rm "$ROOTFS/samples/named.txt"
# Two gzip members back to back: zcat prints both
cat "$ROOTFS/samples/hello.gz" "$ROOTFS/samples/named.gz" > "$ROOTFS/samples/multi.gz"

# More zcat samples, written with python3 (skipped when it is missing):
#   stored.gz, stored-big.gz  uncompressed blocks; the big one is 77000 bytes in two blocks
#   repeat.gz, fixed-big.gz   fixed Huffman code with back-references; the big one is
#                             92000 bytes, so matches reach far back and the window wraps
#   dynamic.gz                dynamic Huffman code, 121695 bytes in several blocks
#   bad-crc.gz, bad-size.gz   valid streams whose trailer does not match
if command -v python3 >/dev/null; then
  python3 - "$ROOTFS/samples" <<'EOF'
import gzip, random, struct, sys, zlib
samples = sys.argv[1]

def gz_fixed(data):
    # raw DEFLATE using only the fixed Huffman code, inside a gzip header and trailer
    packer = zlib.compressobj(9, zlib.DEFLATED, -15, 9, zlib.Z_FIXED)
    raw = packer.compress(data) + packer.flush()
    header = b"\x1f\x8b\x08\x00\x00\x00\x00\x00\x02\xff"
    return header + raw + struct.pack("<II", zlib.crc32(data), len(data))

small = b"stored block works\n" * 3
big = b"".join(b"line %05d\n" % i for i in range(7000))
open(samples + "/stored.gz", "wb").write(gzip.compress(small, compresslevel=0, mtime=0))
open(samples + "/stored-big.gz", "wb").write(gzip.compress(big, compresslevel=0, mtime=0))

repeat = (b"abcabcabcabcabcabc\n"
          b"the quick brown fox. the quick brown fox. the quick brown fox.\n"
          b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n")
rng = random.Random(1)
unit = b"".join(b"%05d %016x\n" % (i, rng.getrandbits(64)) for i in range(800))
open(samples + "/repeat.gz", "wb").write(gz_fixed(repeat))
open(samples + "/fixed-big.gz", "wb").write(gz_fixed(unit * 5))

words_rng = random.Random(2)
words = [b"alpha", b"beta", b"gamma", b"delta", b"epsilon", b"zeta", b"eta", b"theta"]
text = b"".join(b"%05d %s\n" % (i, b" ".join(words_rng.choice(words) for _ in range(6))) for i in range(3000))
open(samples + "/dynamic.gz", "wb").write(gzip.compress(text, 9, mtime=0))

good = gz_fixed(repeat)
open(samples + "/bad-crc.gz", "wb").write(good[:-8] + bytes([good[-8] ^ 1]) + good[-7:])
open(samples + "/bad-size.gz", "wb").write(good[:-1] + bytes([good[-1] ^ 1]))
EOF
fi

# Pack the rootfs as an uncompressed newc cpio archive
(cd "$ROOTFS" && find . | cpio -o -H newc 2>/dev/null) > esp/initrd

echo "[OK] esp/initrd rebuilt ($(wc -c < esp/initrd | tr -d ' ') bytes)"
