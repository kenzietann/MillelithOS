#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Halt the CPU in an infinite spin loop on unrecoverable panics
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

// Opaque pointer to firmware-managed objects
pub type EfiHandle = *mut core::ffi::c_void;

// Native status code integer (UINTN in 64-bit UEFI)
pub type EfiStatus = usize;

// Standard success status code
pub const EFI_SUCCESS: EfiStatus = 0;

// Standard 24-byte header at the start of all UEFI tables
#[repr(C)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

// Protocol interface for printing text to the firmware screen console
#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    pub reset: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
        extended_verification: u8,
    ) -> EfiStatus,
    pub output_string: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
        string: *const u16,
    ) -> EfiStatus,
}

// Master table passed by UEFI firmware to the bootloader entry point
#[repr(C)]
pub struct EfiSystemTable {
    pub hdr: EfiTableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: EfiHandle,
    pub con_in: *mut core::ffi::c_void,
    pub console_out_handle: EfiHandle,
    pub con_out: *mut EfiSimpleTextOutputProtocol,
    pub standard_error_handle: EfiHandle,
    pub std_err: *mut EfiSimpleTextOutputProtocol,
    pub runtime_services: *mut core::ffi::c_void,
    pub boot_services: *mut core::ffi::c_void,
    pub number_of_table_entries: usize,
    pub configuration_table: *mut core::ffi::c_void,
}

// Startup message in UTF-16 terminated with a null byte 0
static MESSAGE: &[u16] = &[
    'H' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16, ' ' as u16,
    'f' as u16, 'r' as u16, 'o' as u16, 'm' as u16, ' ' as u16,
    'M' as u16, 'i' as u16, 'l' as u16, 'l' as u16, 'e' as u16, 'l' as u16, 'i' as u16, 't' as u16, 'h' as u16, ' ' as u16,
    'O' as u16, 'S' as u16, '!' as u16, '\r' as u16, '\n' as u16, 0,
];

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(_image_handle: EfiHandle, system_table: *mut EfiSystemTable) -> EfiStatus {
  let con_out: *mut EfiSimpleTextOutputProtocol = unsafe { (*system_table).con_out };

  unsafe { ((*con_out).output_string)(con_out, MESSAGE.as_ptr()); };

  loop {
    core::hint::spin_loop();
  }
  
}