#![no_std]
#![no_main]

use coreutils::syscall::{OPEN_READ_ONLY, SYS_OPEN, close, open, rename, syscall3, unlink};
use coreutils::{arg, copy_fd, eprint, is_dir, resolve_target};

const EXDEV: isize = -18;
const OPEN_WRITE_ONLY: usize = 1;
const OPEN_CREATE: usize = 0o100;
const OPEN_TRUNCATE: usize = 0o1000;

// rename() cannot cross filesystems, so copy the file and remove the source
// only after the copy and both closes succeeded.
fn move_across_filesystems(source: &[u8], target: &[u8]) -> bool {
  if is_dir(source) {
    eprint("mv: cannot move directory across filesystems\n");
    return false;
  }

  let source_fd = open(source, OPEN_READ_ONLY);
  if source_fd < 0 {
    eprint("mv: cannot open source\n");
    return false;
  }

  let target_fd = unsafe {
    syscall3(
      SYS_OPEN,
      target.as_ptr() as usize,
      OPEN_WRITE_ONLY | OPEN_CREATE | OPEN_TRUNCATE,
      0o644
    )
  };

  if target_fd < 0 {
    close(source_fd as usize);
    eprint("mv: cannot create destination\n");
    return false;
  }

  let copied = copy_fd(source_fd as usize, target_fd as usize);
  let target_close = close(target_fd as usize);
  let source_close = close(source_fd as usize);

  if !copied || target_close < 0 || source_close < 0 {
    unlink(target);
    eprint("mv: copy failed, source left in place\n");
    return false;
  }

  if unlink(source) < 0 {
    eprint("mv: copied, but cannot remove source\n");
    return false;
  }

  true
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

  let Some(target) = resolve_target(source, destination, &mut path_buffer) else {
    eprint("mv: invalid destination path\n");
    return 1;
  };

  let result = rename(source, target);

  if result == EXDEV {
    if move_across_filesystems(source, target) {
      return 0;
    }
    return 1;
  }

  if result < 0 {
    eprint("mv: cannot move path\n");
    return 1;
  }

  return 0;
}
