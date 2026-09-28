#pragma once

#define EFIAPI __attribute__((ms_abi))

using U8 = unsigned char;
using U32 = unsigned int;
using U64 = unsigned long long;

using EFI_STATUS = U64;
using EFI_HANDLE = void*;

struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;

using EFI_RESET = EFI_STATUS (EFIAPI *)(
  EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL*,
  U8
);

using EFI_OUTPUT_STRING = EFI_STATUS (EFIAPI *)(
  EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL*,
  char16_t*
);

// Describes the beginning of UEFI's text-output protocol table.
struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
  EFI_RESET Reset;
  EFI_OUTPUT_STRING OutputString;
};

struct EFI_TABLE_HEADER {
  U64 Signature;
  U32 Revision;
  U32 HeaderSize;
  U32 CRC32;
  U32 Reserved;
};

struct EFI_SYSTEM_TABLE {
  EFI_TABLE_HEADER Hdr;
  char16_t* FirmwareVendor;
  U32 FirmwareRevision;
  EFI_HANDLE ConsoleInHandle;
  void* ConIn;
  EFI_HANDLE ConsoleOutHandle;
  EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL* ConOut;
};

// Verify the type sizes and field layout required by x86-64 UEFI.
static_assert(sizeof(void*) == 8);
static_assert(sizeof(char16_t) == 2);
static_assert(sizeof(EFI_TABLE_HEADER) == 24);
static_assert(__builtin_offsetof(EFI_SYSTEM_TABLE, ConOut) == 64);
