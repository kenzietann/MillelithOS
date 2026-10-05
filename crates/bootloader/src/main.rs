#![no_std]
#![no_main]
mod uefi;
mod graphics;

use uefi::*;
use graphics::*;
use core::panic::PanicInfo;

// Halt the CPU in an infinite spin loop on unrecoverable panics
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

// Startup message in UTF-16 terminated with a null byte 0
static MESSAGE: &[u16] = &[
    'H' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16, ' ' as u16,
    'f' as u16, 'r' as u16, 'o' as u16, 'm' as u16, ' ' as u16,
    'M' as u16, 'i' as u16, 'l' as u16, 'l' as u16, 'e' as u16, 'l' as u16, 'i' as u16, 't' as u16, 'h' as u16, ' ' as u16,
    'O' as u16, 'S' as u16, '!' as u16, '\r' as u16, '\n' as u16, 0,
];



// UEFI application entry point called by the firmware
#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(_image_handle: EfiHandle, system_table: *mut EfiSystemTable) -> EfiStatus {
  let con_out: *mut EfiSimpleTextOutputProtocol = unsafe { (*system_table).con_out };
  let boot_services = unsafe { (*system_table).boot_services };



  // Locate the Graphics Output Protocol
  let mut gop_interface: *mut core::ffi::c_void = core::ptr::null_mut();
  let status = unsafe {
    ((*boot_services).locate_protocol)(
      &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
      core::ptr::null_mut(),
      &mut gop_interface
    )
  };

  if status == EFI_SUCCESS && !gop_interface.is_null(){
    let gop = gop_interface as *mut EfiGraphicsOutputProtocol;

    unsafe { fill_framebuffer(gop, 255, 0, 0) };
  }

  // Print text message after framebuffer filled.
  unsafe { ((*con_out).output_string)(con_out, MESSAGE.as_ptr()); };

  loop {
    core::hint::spin_loop();
  }

}