#![allow(dead_code)]

// Opaque pointer to firmware-managed objects
pub type EfiHandle = *mut core::ffi::c_void;

// Native status code integer (UINTN in 64-bit UEFI)
pub type EfiStatus = usize;

// Standard success status code
pub const EFI_SUCCESS: EfiStatus = 0;

// 128-bit identifier used by UEFI to uniquely distinguish protocols and hardware interfaces
#[repr(C)]
pub struct EfiGuid {
  pub data1: u32,
  pub data2: u16,
  pub data3: u16,
  pub data4: [u8; 8],
}

// Official UEFI Graphics Output Protocol (GOP) GUID: 9042a9de-23dc-4a38-96fb-7ade-d080516a
pub const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data1: 0x9042a9de,
    data2: 0x23dc,
    data3: 0x4a38,
    data4: [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
};

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

// Pixel color encoding format supported by the graphics hardware
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EfiGraphicsPixelFormat {
  PixelRedGreenBlueReserved8BitPerColor = 0,
  PixelBlueGreenRedReserved8BitPerColor = 1,
  PixelBitMask = 2,
  PixelBltOnly = 3,
  PixelFormatMax,
}

// Bitmask configuration for custom pixel formats
#[repr(C)]
pub struct EfiPixelBitmask {
  pub red_mask: u32,
  pub green_mask: u32,
  pub blue_mask: u32,
  pub reserved_mask: u32,
}

// Information describing screen resolution and line stride
#[repr(C)]
pub struct EfiGraphicsOutputModeInformation {
  pub version: u32,
  pub horizontal_resolution: u32,
  pub vertical_resolution: u32,
  pub pixel_format: EfiGraphicsPixelFormat,
  pub pixel_information: EfiPixelBitmask,
  pub pixels_per_scan_line: u32,
}

// Runtime state of the graphics hardware including the physical framebuffer address
#[repr(C)]
pub struct EfiGraphicsOutputProtocolMode {
  pub max_mode: u32,
  pub mode: u32,
  pub info: *mut EfiGraphicsOutputModeInformation,
  pub size_of_info: usize,
  pub frame_buffer_base: u64,
  pub frame_buffer_size: usize,
}

// Protocol interface for hardware accelerated or direct framebuffer graphics
#[repr(C)]
pub struct EfiGraphicsOutputProtocol {
  pub query_mode: usize,
  pub set_mode: usize,
  pub blt: usize,
  pub mode: *mut EfiGraphicsOutputProtocolMode,
}

// Services provided by UEFI firmware while booting before the OS takes full control
#[repr(C)]
pub struct EfiBootServices {
    pub hdr: EfiTableHeader,
    // Task priority services
    pub raise_tpl: usize,
    pub restore_tpl: usize,
    // Memory allocation services
    pub allocate_pages: unsafe extern "efiapi" fn(
      alloc_type: u32,
      memory_type: u32,
      pages: usize,
      memory: *mut u64,
    ) -> EfiStatus,
    pub free_pages: usize,
    pub get_memory_map: usize,
    pub allocate_pool: usize,
    pub free_pool: usize,
    // Event & timer services
    pub create_event: usize,
    pub set_timer: usize,
    pub wait_for_event: usize,
    pub signal_event: usize,
    pub close_event: usize,
    pub check_event: usize,
    // Protocol interface services
    pub install_protocol_interface: usize,
    pub reinstall_protocol_interface: usize,
    pub uninstall_protocol_interface: usize,
    pub handle_protocol: unsafe extern "efiapi" fn(
      handle: EfiHandle,
      protocol: *const EfiGuid,
      interface: *mut *mut core::ffi::c_void,
    ) -> EfiStatus,
    pub reserved: usize,
    pub register_protocol_notify: usize,
    pub locate_handle: usize,
    pub locate_device_path: usize,
    pub install_configuration_table: usize,
    // Image loading services
    pub image_load: unsafe extern "efiapi" fn(
        boot_policy: u8,
        parent_image_handle: EfiHandle,
        device_path: *mut core::ffi::c_void,
        source_buffer: *mut core::ffi::c_void,
        source_size: usize,
        image_handle: *mut EfiHandle,
    ) -> EfiStatus,
    pub image_start:unsafe extern "efiapi" fn(
        image_handle: EfiHandle,
        exit_data_size: *mut usize,
        exit_data: *mut *mut u16,
    ) -> EfiStatus,
    pub exit: usize,
    pub image_unload: usize,
    pub exit_boot_services: unsafe extern "efiapi" fn(
      image_handle: EfiHandle,
      map_key: usize,
    ) -> EfiStatus,
    // Miscellaneous services
    pub get_next_monotonic_count: usize,
    pub stall: usize,
    pub set_watchdog_timer: usize,
    // Driver support services
    pub connect_controller: usize,
    pub disconnect_controller: usize,
    // Open/close protocol services
    pub open_protocol: usize,
    pub close_protocol: usize,
    pub open_protocol_information: usize,
    // Library services
    pub protocols_per_handle: usize,
    pub locate_handle_buffer: usize,
    // Protocol lookup: find a protocol by its GUID
    pub locate_protocol: unsafe extern "efiapi" fn(
        protocol: *const EfiGuid,
        registration: *mut core::ffi::c_void,
        interface: *mut *mut core::ffi::c_void, 
    ) -> EfiStatus,
}

// Allocate memory at any available physical address below max
pub const EFI_ALLOCATE_ANY_PAGES: u32 = 0;
// Loader data memory type (freed or reclaimed by OS after boot)
pub const EFI_LOADER_DATA: u32 = 2;
// Page size in bytes (4 KiB)
pub const EFI_PAGE_SIZE: usize = 4096;

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
    pub boot_services: *mut EfiBootServices,
    pub number_of_table_entries: usize,
    pub configuration_table: *mut core::ffi::c_void,
}

// GUID for UEFI Simple File System Protocol: 964e5b22-6459-11d2-8e39-00a0c969723b
pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data1: 0x964e5b22,
    data2: 0x6459,
    data3: 0x11d2,
    data4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
};

#[repr(C)]
pub struct EfiFileProtocol {
  pub revision: u64,
  
  // Open a file relative to this directory
  pub open: unsafe extern "efiapi" fn(
    this: *mut EfiFileProtocol,
    new_handle: *mut *mut EfiFileProtocol,
    file_name: *const u16,
    open_mode: u64,
    attributes: u64
  ) -> EfiStatus,

  // Close the file handle
  pub close: unsafe extern "efiapi" fn(this: *mut EfiFileProtocol) -> EfiStatus,
  pub delete: usize,

  // Read data from the file into a memory buffer
  pub read: unsafe extern "efiapi" fn(
    this: *mut EfiFileProtocol,
    buffer_size: *mut usize,
    buffer: *mut core::ffi::c_void
  ) -> EfiStatus,
  pub write: usize,
  pub get_position: unsafe extern "efiapi" fn(
    this: *mut EfiFileProtocol,
    position: *mut u64,
  ) -> EfiStatus,
  pub set_position: unsafe extern "efiapi" fn(
    this: *mut EfiFileProtocol,
    position: u64
  ) -> EfiStatus,
  pub get_info: usize,
  pub set_info: usize,
  pub flush: usize,
}

// Protocol used to access a FAT file system volume
#[repr(C)]
pub struct EfiSimpleFileSystemProtocol {
  pub revision: u64,

  // Open the root directory of the volume
  pub open_volume: unsafe extern "efiapi" fn(
    this: *mut EfiSimpleFileSystemProtocol,
    root: *mut *mut EfiFileProtocol
  ) -> EfiStatus
}

pub const EFI_FILE_MODE_READ: u64 = 0x0000000000000001;

// GUID for UEFI Loaded Image Protocol: 5b1b31a1-9562-11d2-8e3f-00a0c969723b
pub const EFI_LOADED_IMAGE_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data1: 0x5b1b31a1,
    data2: 0x9562,
    data3: 0x11d2,
    data4: [0x8e, 0x3f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
};

// Layout for an executing UEFI binary image interface
#[repr(C)]
pub struct LoadedImageConfig {
    pub revision: u32,
    pub parent: EfiHandle,
    pub system_table: *mut EfiSystemTable,
    pub device: EfiHandle,
    pub file_path: *mut core::ffi::c_void,
    pub reserved: *mut core::ffi::c_void,
    pub options_bytes: u32,
    pub options_ptr: *mut u16,
    pub image_base: *mut core::ffi::c_void,
    pub image_size: u64,
    pub image_code_type: u32,
    pub image_data_type: u32,
    pub unload_handler: usize,
}