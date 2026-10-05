use crate::uefi::*;

// Fill the entire visible GOP framebuffer with an RGB color
pub unsafe fn fill_framebuffer(gop: *mut EfiGraphicsOutputProtocol, red: u8, green: u8, blue: u8) -> EfiStatus {
  unsafe {
    if gop.is_null() || (*gop).mode.is_null() {
      return 1;
    }
  
    let mode: *mut EfiGraphicsOutputProtocolMode = (*gop).mode;
    let info: *mut EfiGraphicsOutputModeInformation = (*mode).info;
    if info.is_null() || (*mode).frame_buffer_base == 0 {
      return 1;
    }
    
    // Determine pixel color encoding based on firmware hardware report
    let pixel: u32 = match (*info).pixel_format {
      EfiGraphicsPixelFormat::PixelRedGreenBlueReserved8BitPerColor => {
        (red as u32) | ((green as u32) << 8) | ((blue as u32) << 16)
      }
  
      EfiGraphicsPixelFormat::PixelBlueGreenRedReserved8BitPerColor => {
        (blue as u32) | ((green as u32) << 8) | ((red as u32) << 16)
      } 
      _ => return 1,
    };
  
    // Raw pointer to the physical video memory
    let fb: *mut u32 = (*mode).frame_buffer_base as *mut u32;
    let stride: usize = (*info).pixels_per_scan_line as usize;
    let width: usize = (*info).horizontal_resolution as usize;
    let height: usize = (*info).vertical_resolution as usize;
  
    // Paint every pixel on the screen row by row
    for y in 0..height {
      for x in 0..width {
        let offset = y * stride + x;
        core::ptr::write_volatile(fb.add(offset), pixel);
      }
    }
  
    EFI_SUCCESS
  }
}