#include "uefi.hpp"
// UTF-16 message passed to UEFI's OutputString service.
static char16_t message[] = u"Hello from OS!\r\n";
static char16_t error_message[] = u"Error: UEFI failed to print the startup message.\r\n";
static char16_t ready_message[] = u"UEFI text output is available.\r\n";

EFI_STATUS print_message(
  EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL* console,
  char16_t* text
) {
  return console->OutputString(console, text);
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

  // Keep the application alive, halting the CPU again after each interrupt.
  for(;;){
    __asm__ volatile("hlt");
  }
}
