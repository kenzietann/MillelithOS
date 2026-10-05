#pragma once

#define EFIAPI __attribute__((ms_abi))

using U8 = unsigned char;
using U16 = unsigned short;
using U32 = unsigned int;
using U64 = unsigned long long;

using EFI_STATUS = U64;
constexpr EFI_STATUS EFI_BUFFER_TOO_SMALL = 0x8000000000000005ULL;
constexpr EFI_STATUS EFI_BAD_BUFFER_SIZE = 0x8000000000000004ULL;
using EFI_HANDLE = void*;
using UINTN = U64;
using EFI_PHYSICAL_ADDRESS = U64;
using EFI_VIRTUAL_ADDRESS = U64;

struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;
struct EFI_RUNTIME_SERVICES;
struct EFI_BOOT_SERVICES;
struct EFI_GRAPHICS_OUTPUT_PROTOCOL;

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

struct alignas(8) EFI_GUID {
  U32 Data1;
  U16 Data2;
  U16 Data3;
  U8 Data4[8];
};

struct EFI_MEMORY_DESCRIPTOR {
  U32 Type;
  EFI_PHYSICAL_ADDRESS PhysicalStart;
  EFI_VIRTUAL_ADDRESS VirtualStart;
  U64 NumberOfPages;
  U64 Attribute;
};

using EFI_GET_MEMORY_MAP = EFI_STATUS (EFIAPI *)(
  UINTN* MemoryMapSize,
  EFI_MEMORY_DESCRIPTOR* MemoryMap,
  UINTN* MapKey,
  UINTN* DescriptorSize,
  U32* DescriptorVersion
);

using EFI_LOCATE_PROTOCOL = EFI_STATUS (EFIAPI *)(
  EFI_GUID* protocol,
  void* registration,
  void** interface
);

struct EFI_BOOT_SERVICES {
  EFI_TABLE_HEADER Hdr;
  U64 ServicesBeforeGetMemoryMap[4];
  EFI_GET_MEMORY_MAP GetMemoryMap;
  U64 ServicesBeforeLocateProtocol[32];
  EFI_LOCATE_PROTOCOL LocateProtocol;
};

enum EFI_GRAPHICS_PIXEL_FORMAT : U32 {
  PixelRedGreenBlueReserved8BitPerColor = 0,
  PixelBlueGreenRedReserved8BitPerColor = 1,
  PixelBitMask = 2,
  PixelBltOnly = 3,
  PixelFormatMax = 4
};

struct EFI_PIXEL_BITMASK {
  U32 RedMask;
  U32 GreenMask;
  U32 BlueMask;
  U32 ReservedMask;
};

struct EFI_GRAPHICS_OUTPUT_MODE_INFORMATION {
  U32 Version;
  U32 HorizontalResolution;
  U32 VerticalResolution;
  EFI_GRAPHICS_PIXEL_FORMAT PixelFormat;
  EFI_PIXEL_BITMASK PixelInformation;
  U32 PixelsPerScanLine;
};

struct EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE {
  U32 MaxMode;
  U32 Mode;
  EFI_GRAPHICS_OUTPUT_MODE_INFORMATION* Info;
  UINTN SizeOfInfo;
  EFI_PHYSICAL_ADDRESS FrameBufferBase;
  UINTN FrameBufferSize;
};

//QueryMode, SetMode and Blt occupy the first three slots.
//We will give them complete function types when we use them.
struct EFI_GRAPHICS_OUTPUT_PROTOCOL {
  U64 ServicesBeforeMode[3];
  EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE* Mode;
};

struct EFI_SYSTEM_TABLE {
  EFI_TABLE_HEADER Hdr;
  char16_t* FirmwareVendor;
  U32 FirmwareRevision;
  EFI_HANDLE ConsoleInHandle;
  void* ConIn;
  EFI_HANDLE ConsoleOutHandle;
  EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL* ConOut;
  EFI_HANDLE StandardErrorHandle;
  EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL* StdErr;
  EFI_RUNTIME_SERVICES* RuntimeServices;
  EFI_BOOT_SERVICES* BootServices;
};

// Verify the type sizes and field layout required by x86-64 UEFI.
static_assert(sizeof(void*) == 8);
static_assert(sizeof(char16_t) == 2);
static_assert(sizeof(EFI_TABLE_HEADER) == 24);
static_assert(__builtin_offsetof(EFI_SYSTEM_TABLE, ConOut) == 64);
static_assert(__builtin_offsetof(EFI_SYSTEM_TABLE, BootServices) == 96);
static_assert(sizeof(EFI_GUID) == 16);
static_assert(alignof(EFI_GUID) == 8);
static_assert(__builtin_offsetof(EFI_BOOT_SERVICES, GetMemoryMap) == 56);
static_assert(__builtin_offsetof(EFI_BOOT_SERVICES, LocateProtocol) == 320);
static_assert(sizeof(EFI_BOOT_SERVICES) == 328);
static_assert(sizeof(EFI_PIXEL_BITMASK) == 16);
static_assert(sizeof(EFI_GRAPHICS_OUTPUT_MODE_INFORMATION) == 36);
static_assert(__builtin_offsetof(EFI_GRAPHICS_OUTPUT_MODE_INFORMATION, PixelsPerScanLine) == 32);
static_assert(sizeof(EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE) == 40);
static_assert(__builtin_offsetof(EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE, FrameBufferBase) == 24);
static_assert(__builtin_offsetof(EFI_GRAPHICS_OUTPUT_PROTOCOL, Mode) == 24);