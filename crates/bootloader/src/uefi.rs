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
    pub allocate_pages: usize,
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
    pub handle_protocol: usize,
    pub reserved: usize,
    pub register_protocol_notify: usize,
    pub locate_handle: usize,
    pub locate_device_path: usize,
    pub install_configuration_table: usize,
    // Image loading services
    pub image_load: usize,
    pub image_start: usize,
    pub exit: usize,
    pub image_unload: usize,
    pub exit_boot_services: usize,
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