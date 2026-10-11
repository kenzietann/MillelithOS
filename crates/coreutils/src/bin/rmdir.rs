#![no_std]
#![no_main]

use coreutils::syscall::rmdir;
use coreutils::{arg, eprint};

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc < 2 {
    eprint("usage: rmdir DIRECTORY...\n");
    return 1;
  }

  let mut status = 0;

  for index in 1..argc {
    let path = arg(argv, index);

    if rmdir(path) < 0 {
      eprint("rmdir: cannot remove directory\n");
      status = 1;
    }
  }

  return status;
}