#![no_std]
#![no_main]

use coreutils::syscall::write;
use coreutils::{arg, print};

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  for index in 1..argc {
    if index> 1 {
      print(" ");
    }

    let text = arg(argv, index);
    write(1, &text[..text.len() - 1]);
  }

  print("\n");
  0
}