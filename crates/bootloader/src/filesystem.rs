use crate::uefi::*;

// Open the root directory of the boot filesystem volume
pub unsafe fn open_root_dir(
    boot_services: *mut EfiBootServices,
) -> Result<*mut EfiFileProtocol, EfiStatus> {
    unsafe {
        // Locate Simple File System protocol interface
        let mut sfs_proto: *mut EfiSimpleFileSystemProtocol = core::ptr::null_mut();
        let status = ((*boot_services).locate_protocol)(
            &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
            core::ptr::null_mut(),
            &mut sfs_proto as *mut _ as *mut *mut core::ffi::c_void,
        );

        if status != EFI_SUCCESS {
            return Err(status);
        }

        // Open root volume directory handle
        let mut root_dir: *mut EfiFileProtocol = core::ptr::null_mut();
        let status = ((*sfs_proto).open_volume)(sfs_proto, &mut root_dir);

        if status != EFI_SUCCESS {
            return Err(status);
        }

        Ok(root_dir)
    }
}

pub unsafe fn open_file(root: *mut EfiFileProtocol, filename: &str) -> Result<*mut EfiFileProtocol, EfiStatus> {
  unsafe {
    // Convert ASCII/UTF-8 filename into a null-terminated UTF-16 buffer
    let mut u16_name = [0u16; 128];
    if filename.len() >= u16_name.len() {
      return Err(1);
    }

    for (i, byte) in filename.bytes().enumerate() {
      u16_name[i] = byte as u16;
    }

    u16_name[filename.len()] = 0;

    let mut file_handle: *mut EfiFileProtocol = core::ptr::null_mut();
    let status = ((*root).open)(
      root,
      &mut file_handle,
      u16_name.as_ptr(),
      EFI_FILE_MODE_READ,
      0
    );

    if status != EFI_SUCCESS {
      return Err(status);
    }

    Ok(file_handle)
  }
}

pub unsafe fn read_file(
  file: *mut EfiFileProtocol,
  buffer: *mut u8,
  size: *mut usize
) -> Result<(), EfiStatus> {
  unsafe {
    let status = ((*file).read)(
      file,
      size,
      buffer as *mut core::ffi::c_void,
    );

    if status != EFI_SUCCESS {
      return Err(status);
    }

    Ok(())
  }
}

// Allocate physical RAM pages and load a file directly into memory
pub unsafe fn load_file_to_memory(
    boot_services: *mut EfiBootServices,
    file: *mut EfiFileProtocol,
    max_pages: usize,
) -> Result<(*mut u8, usize), EfiStatus> {
    unsafe {
        // Allocate physical RAM pages from UEFI
        let mut memory_addr: u64 = 0;
        let status = ((*boot_services).allocate_pages)(
            EFI_ALLOCATE_ANY_PAGES,
            EFI_LOADER_DATA,
            max_pages,
            &mut memory_addr,
        );

        if status != EFI_SUCCESS {
            return Err(status);
        }

        let buffer_ptr = memory_addr as *mut u8;
        let mut read_size: usize = max_pages * EFI_PAGE_SIZE;

        // Firmware reads file data into RAM and updates read_size to actual bytes read
        let read_status = read_file(file, buffer_ptr, &mut read_size);
        if read_status.is_err() {
            return Err(read_status.unwrap_err());
        }

        Ok((buffer_ptr, read_size))
    }
}