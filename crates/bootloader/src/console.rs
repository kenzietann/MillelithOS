use crate::uefi::*;

// Print an ASCII/UTF-8 string slice to the UEFI text console
pub fn print_str(con_out: *mut EfiSimpleTextOutputProtocol, text: &str) {
  if con_out.is_null() {
    return;
  }

  // Small buffer for converting UTF-8 to UCS-2 / UTF-16 on the fly
  let mut buf = [0u16; 2];

  for byte in text.bytes() {
    // Handle newline by emitting carriage return first

    if byte == b'\n' {
      buf[0] = b'\r' as u16;
      buf[1] = 0;
      unsafe {
        ((*con_out).output_string)(con_out, buf.as_ptr());
      }
    }

    buf[0] = byte as u16;
    buf[1] = 0;
    unsafe {
      ((*con_out).output_string)(con_out, buf.as_ptr());
    }
  }
}

pub fn print_hex(con_out: *mut EfiSimpleTextOutputProtocol, val: u64) {
  const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";
  let mut buf = [b'0'; 18];
  buf[0] = b'0';
  buf[1] = b'x';

  for i in 0..16 {
    let shift = (15 - i) * 4;
    let nibble = ((val >> shift) & 0xF) as usize;
    buf[2 + i] = HEX_CHARS[nibble];
  }

  // Convert buffer to ASCII str and print
  if let Ok(s) = core::str::from_utf8(&buf) {
    print_str(con_out, s);
  }
}

pub fn print_ptr<T>(con_out: *mut EfiSimpleTextOutputProtocol, ptr: *const T){
  print_hex(con_out, ptr as u64);
}