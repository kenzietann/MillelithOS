#![no_std]
#![no_main]

use coreutils::syscall::rename;
use coreutils::{arg, eprint, resolve_target};

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc != 3 {
    eprint("usage: mv SOURCE DEST\n");
    return 1;
  }

  let source = arg(argv, 1);
  let destination = arg(argv, 2);
  let mut path_buffer = [0u8; 4096];

  let Some(target) = resolve_target(source, destination, &mut path_buffer) else {
    eprint("mv: invalid destination path\n");
    return 1;
  };

  if rename(source, target) < 0 {
    eprint("mv: cannot move path\n");
    return 1;
  }

  return 0;
}
