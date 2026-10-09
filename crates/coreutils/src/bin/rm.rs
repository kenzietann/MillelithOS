#![no_std]
#![no_main]

use coreutils::syscall::unlink;
use coreutils::{arg, eprint};

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc < 2 {
    eprint("usage: fm FILE...\n");
    return 1;
  }

  let mut status = 0;

  for index in 1..argc {
    let path = arg(argv, index);

    if unlink(path) < 0 {
      eprint("rm: cannot remove file\n");
      status = 1;
    }
  }
  
  return status;
}