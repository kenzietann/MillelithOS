#![no_std]
#![no_main]

use coreutils::syscall::{S_IFDIR, S_IFMT, rename, stat};
use coreutils::{arg, eprint};

fn is_dir(path: &[u8]) -> bool {
  let mut buf = [0u8; 144];
  if stat(path, &mut buf) < 0 {
    return false;
  }

  let mode = u32::from_le_bytes([buf[24], buf[25], buf[26], buf[27]]);
  mode & S_IFMT == S_IFDIR
}

#[unsafe(no_mangle)]
pub extern "C" fn main_entry(argc: usize, argv: *const *const u8) -> usize {

  if argc != 3 {
    eprint("usage: mv SOURCE DEST\n");
    return 1;
  }

  let source = arg(argv, 1);
  let destination = arg(argv, 2);
  let mut path_buffer = [0u8; 4096];

  let has_slash = destination.len() >= 2 && destination[destination.len() - 2] == b'/';
  let into_dir = has_slash || is_dir(destination);

  let target: &[u8] = if into_dir {
    let source_name = &source[..source.len() - 1];
    let mut name_start = 0;

    for index in 0..source_name.len() {
      if source_name[index] == b'/' {
        name_start = index + 1;
      }
    }

    let basename = &source_name[name_start..];
    let prefix = &destination[..destination.len() - 1];
    let needs_slash = !has_slash;
    let slash_len = needs_slash as usize;
    let total = prefix.len() + slash_len + basename.len() + 1;

    if basename.is_empty() || total > path_buffer.len() {
      eprint("mv: invalid destination path\n");
      return 1;
    }

    path_buffer[..prefix.len()].copy_from_slice(prefix);
    if needs_slash {
      path_buffer[prefix.len()] = b'/';
    }
    let name_offset = prefix.len() + slash_len;
    path_buffer[name_offset..name_offset + basename.len()].copy_from_slice(basename);
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