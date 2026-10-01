#include "uefi.hpp"
// UTF-16 message passed to UEFI's OutputString service.
static char16_t message[] = u"Hello from OS!\r\n";
static char16_t error_message[] = u"Error: UEFI failed to print the startup message.\r\n";
static char16_t ready_message[] = u"UEFI text output is available.\r\n";

static EFI_GUID graphics_output_protocol_guid = {
  0x9042a9de,
  0x23dc,
  0x4a38,
  {
    0x96, 0xfb, 0x7a, 0xde,
    0xd0, 0x80, 0x51, 0x6a
  }
};

static char16_t graphics_ready_message[] = u"UEFI graphics output is available.\r\n";
static char16_t graphics_error_message[] = u"Error: UEFI graphics output is unavailable\r\n";
static char16_t framebuffer_ready_message[] = u"UEFI Framebuffer is available.\r\n";
static char16_t framebuffer_error_message[] = u"Error: no writable UEFI framebuffer is available.\r\n";

EFI_STATUS print_message(
  EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL* console,
  char16_t* text
) {
  return console->OutputString(console, text);
}

EFI_STATUS fill_framebuffer(EFI_GRAPHICS_OUTPUT_PROTOCOL* graphics, U8 red, U8 green, U8 blue) {
  if (graphics == nullptr ||
      graphics->Mode == nullptr ||
      graphics->Mode->Info == nullptr ||  
      graphics->Mode->FrameBufferBase == 0 ||
      graphics->Mode->FrameBufferSize == 0) {
    return 0x8000000000000002ULL;
  }

  EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE* mode = graphics->Mode;
  EFI_GRAPHICS_OUTPUT_MODE_INFORMATION* info = mode->Info;

  if(info->HorizontalResolution == 0 ||
     info->VerticalResolution == 0 ||
     info->PixelsPerScanLine < info->HorizontalResolution) {
    return 0x8000000000000002ULL;
  }

  U32 pixel = 0;

  if(info->PixelFormat == PixelRedGreenBlueReserved8BitPerColor) {
    pixel = static_cast<U32>(red) | (static_cast<U32>(green) << 8) | (static_cast<U32>(blue) << 16);
  } else if (
    info->PixelFormat == PixelBlueGreenRedReserved8BitPerColor
  ) {
    pixel = static_cast<U32>(blue) | (static_cast<U32>(green) << 8) | (static_cast<U32>(red) << 16);
  } else {
    return 0x8000000000000003ULL;
  }

  UINTN required_bytes = static_cast<UINTN>(info->PixelsPerScanLine) * static_cast<UINTN>(info->VerticalResolution) * sizeof(U32);

  if(required_bytes > mode->FrameBufferSize){
    return 0x8000000000000004ULL;
  }

  volatile U32* framebuffer = reinterpret_cast<volatile U32*>(mode->FrameBufferBase);

  for(UINTN y = 0; y < info->VerticalResolution; ++y){
    for(UINTN x = 0; x < info->HorizontalResolution; ++x){
      UINTN pixel_index = y * info->PixelsPerScanLine + x;
      framebuffer[pixel_index] = pixel;
    }
  }

  return 0;
}

// UEFI Application entry point. Use C naming and the
// x86-64 UEFI calling convention.
extern "C" EFI_STATUS EFIAPI EfiMain(EFI_HANDLE image_handle, EFI_SYSTEM_TABLE* system_table){
  (void)image_handle;

  // Verify that the system table, console protocol,
  // and printing function are available.
  if(system_table == nullptr ||
     system_table->ConOut == nullptr ||
     system_table->ConOut->OutputString == nullptr){
    return 0x8000000000000002ULL;
  }

  // Ask UEFI's text-output protocol to display the message.
  EFI_STATUS status = print_message(system_table->ConOut, message);

  // Return any printing error to the firmware.
  if(status != 0){
    print_message(system_table->ConOut, error_message);

    return status;
  }

  status = print_message(system_table->ConOut, ready_message);
  if(status != 0){
    print_message(system_table->ConOut, error_message);

    return status;
  }

  if (system_table->BootServices == nullptr ||
      system_table->BootServices->LocateProtocol == nullptr) {
    print_message(system_table->ConOut, graphics_error_message);

    return 0x8000000000000003ULL;
  }

  void* graphics_output_interface = nullptr;

  status = system_table->BootServices->LocateProtocol(
    &graphics_output_protocol_guid,
    nullptr,
    &graphics_output_interface
  );

  if (status != 0) {
    print_message(system_table->ConOut, graphics_error_message);

    return status;
  }

  if (graphics_output_interface == nullptr) {
    print_message(system_table->ConOut, graphics_error_message);

    return 0x8000000000000003ULL;
  }

  EFI_GRAPHICS_OUTPUT_PROTOCOL* graphics_output =
    static_cast<EFI_GRAPHICS_OUTPUT_PROTOCOL*>(
      graphics_output_interface
    );

  status = print_message(system_table->ConOut, graphics_ready_message);

  if(status != 0){
    return status;
  }

  if (graphics_output->Mode == nullptr ||
      graphics_output->Mode->Info == nullptr ||
      graphics_output->Mode->FrameBufferBase == 0 ||
      graphics_output->Mode->FrameBufferSize == 0 ||
      graphics_output->Mode->Info->PixelFormat == PixelBltOnly) {
    print_message(
      system_table->ConOut,
      framebuffer_error_message
    );

    return 0x8000000000000003ULL;
  }

  status = print_message(
    system_table->ConOut,
    framebuffer_ready_message
  );

  if (status != 0) {
    return status;
  }

  status = fill_framebuffer(graphics_output,24,65,110);

  if(status != 0){
    print_message(system_table->ConOut, framebuffer_error_message);

    return status;
  }

  // Keep the application alive, halting the CPU again after each interrupt.
  for(;;){
    __asm__ volatile("hlt");
  }
}
