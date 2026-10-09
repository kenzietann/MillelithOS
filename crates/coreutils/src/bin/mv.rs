#![no_std]
#![no_main]

use coreutils::syscall::rename;
use coreutils::{arg, eprint};

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {
  if argc != 3 {
    eprint("usage: mv SOURCE DEST\n");
    return 1;
  }

  let source = arg(argv, 1);
  let destination = arg(argv, 2);
  let mut path_buffer = [0u8; 4096];

  let target: &[u8] = if destination.len() >= 2 && destination[destination.len() - 2] == b'/' {
    let source_name = &source[..source.len() - 1];
    let mut name_start = 0;

    for index in 0..source_name.len() {
      if source_name[index] == b'/' {
        name_start = index + 1;
      }
    }

    let basename = &source_name[name_start..];
    let prefix = &destination[..destination.len() - 1];
    let total = prefix.len() + basename.len() + 1;

    if basename.is_empty() || total > path_buffer.len() {
      eprint("mv: invalid destination path\n");
      return 1;
    }

    path_buffer[..prefix.len()].copy_from_slice(prefix);
    path_buffer[prefix.len()..prefix.len() + basename.len()].copy_from_slice(basename);
    path_buffer[total - 1] = 0;

    &path_buffer[..total]
  } else {
    destination
  };

  if rename(source, target) < 0 {
    eprint("mv: cannot move path\n");
    return 1;
  }

  return 0;
}