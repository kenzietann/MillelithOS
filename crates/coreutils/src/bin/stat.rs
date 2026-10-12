#![no_std]
#![no_main]

use coreutils::syscall::stat;
use coreutils::{arg, eprint, print, print_u64};

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> isize {
  if argc != 2 {
    eprint("usage: stat FILE\n");
    return 1;
  }

  let path = arg(argv, 1);
  let mut buf = [0u8; 144];

  if stat(path, &mut buf) < 0 {
    eprint("stat: cannot stat file\n");
    return 1;
  }

  let size = u64::from_le_bytes([
    buf[48], buf[49], buf[50], buf[51],
    buf[52], buf[53], buf[54], buf[55],
  ]);

  print("size: ");    
  print_u64(size);
  print("\n");

  let mtime = u64::from_le_bytes([
    buf[88], buf[89], buf[90], buf[91],
    buf[92], buf[93], buf[94], buf[95],
  ]);

  print("mtime: ");
  print_u64(mtime);
  print("\n");

  return 0;
}