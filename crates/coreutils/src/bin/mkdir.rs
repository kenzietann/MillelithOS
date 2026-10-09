#![no_std]
#![no_main]

use coreutils::syscall::mkdir;
use coreutils::{arg, eprint};

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc != 2 {
    eprint("usage: mkdir DIRECTORY\n");
    return 1;
  }

  let path = arg(argv, 1);

  if mkdir(path, 0o755) < 0 {
    eprint("mkdir: cannot create directory\n");
    return 1;
  }

  return 0;
}