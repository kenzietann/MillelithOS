#![no_std]
#![no_main]

mod uefi;
mod graphics;
mod filesystem;
mod console;

use uefi::*;
use graphics::*;
use console::*;

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

    unsafe { fill_framebuffer(gop, 0, 0, 0) };
  }

  // Print text message after framebuffer filled.
  unsafe { ((*con_out).output_string)(con_out, MESSAGE.as_ptr()); };

  let root_dir: Result<*mut EfiFileProtocol, usize> = unsafe {
    filesystem::open_root_dir(boot_services) 
  };

  match root_dir {
    Ok(root) => {
      print_str(con_out, "[OK] Root dir handle at: ");
      print_ptr(con_out, root);
      print_str(con_out, "\n");

      print_str(con_out, "[*] Loading kernel: vmlinuz...\n");

      // Buka file vmlinuz dari partisi boot
      let kernel_file = unsafe { filesystem::open_file(root, "vmlinuz") };
      match kernel_file {
        Ok(handle) => {
          print_str(con_out, "[OK] Kernel file opened successfully!\n");

          // Alokasikan physical pages dan muat seluruh kernel 14MB ke RAM
          let load_result = unsafe {
            filesystem::load_file_to_memory(boot_services, handle, 4096)
          };

          match load_result {
            Ok((kernel_buffer, kernel_size)) => {
              // Verify Linux bzImage signature ("HdrS" at offset 0x202)
              let magic = unsafe {
                  core::ptr::read_unaligned(kernel_buffer.add(0x202) as *const u32)
              };

              // 0x53726448 is ASCII "HdrS" in Little Endian
              if magic == 0x5372_6448 {
                  print_str(con_out, "[OK] Valid Linux bzImage signature ('HdrS') confirmed!\n");

                  // Read 64-bit EFI handover offset at offset 0x264
                  let handover_offset = unsafe {
                      core::ptr::read_unaligned(kernel_buffer.add(0x264) as *const u32)
                  };

                  print_str(con_out, "[OK] EFI 64-bit Handover Offset: ");
                  print_hex(con_out, handover_offset as u64);
                  print_str(con_out, "\n");

                  print_str(con_out, "[*] Loading Linux Kernel image via EFI stub...\n");

                  let mut kernel_image_handle: EfiHandle = core::ptr::null_mut();
                  let status = unsafe {
                      ((*boot_services).image_load)(
                          0,
                          _image_handle,
                          core::ptr::null_mut(),
                          kernel_buffer as *mut core::ffi::c_void,
                          kernel_size,
                          &mut kernel_image_handle,
                      )
                  };

                  if status != EFI_SUCCESS {
                      print_str(con_out, "[ERROR] image_load failed with status: ");
                      print_hex(con_out, status as u64);
                      print_str(con_out, "\n");
                      loop { core::hint::spin_loop(); }
                  }

                  print_str(con_out, "[OK] Kernel image loaded! Handle at: ");
                  print_ptr(con_out, kernel_image_handle);
                  print_str(con_out, "\n[*] Starting Linux Kernel...\n");

                  let mut exit_data_size: usize = 0;
                  let mut exit_data: *mut u16 = core::ptr::null_mut();

                  let status = unsafe {
                      ((*boot_services).image_start)(
                          kernel_image_handle,
                          &mut exit_data_size,
                          &mut exit_data,
                      )
                  };

                  if status != EFI_SUCCESS {
                      print_str(con_out, "[ERROR] image_start failed with status: ");
                      print_hex(con_out, status as u64);
                      print_str(con_out, "\n");
                  }

              } else {
                  print_str(con_out, "[ERROR] Invalid kernel image signature!\n");
              }
              print_str(con_out, "[OK] Kernel loaded into RAM at: ");
              print_ptr(con_out, kernel_buffer);
              print_str(con_out, "\n[OK] Kernel size (bytes): ");
              print_hex(con_out, kernel_size as u64);
              print_str(con_out, "\n");
            }
            Err(status) => {
              print_str(con_out, "[ERROR] Failed to load kernel to memory, status: ");
              print_hex(con_out, status as u64);
              print_str(con_out, "\n");
            }
          }
        }
        Err(status) => {
          print_str(con_out, "[ERROR] vmlinuz not found on boot partition, status: ");
          print_hex(con_out, status as u64);
          print_str(con_out, "\n");
        }
      }
    }
    Err(status) => {
      print_str(con_out, "[ERROR] Failed to open root directory, status: ");
      print_hex(con_out, status as u64);
      print_str(con_out, "\n");
    }
  }


  loop {
    core::hint::spin_loop();
  }

}