/**
 * UEFI PE32+ Header for RiscV64
 */

    .section .text.head

    .globl ImageBase
ImageBase:
    /* DOS stub */
    .short 0x5A4D /* DOS Magic: 'MZ' */
    .skip 58 /* DOS header length is 64 bytes */
    .long pe_header - ImageBase /* pointer to PE header */
pe_header:
    .long 0x00004550 /* PE Magic: 'PE\0\0' */
    .short 0x5064 /* Machine: EFI_IMAGE_MACHINE_RISCV64 */
    .short 3 /* NumberOfSections */
    .long 0 /* TimeDateStamp */
    .long 0 /* PointerToSymbolTable */
    .long 0 /* NumberOfSymbols */
    .short section_table - optional_header /* SizeOfOptionalHeader */
    .short 0x020e /* Characteristics */
        /* 0x0002 - IMAGE_FILE_EXECUTABLE_IMAGE */
        /* 0x0004 - IMAGE_FILE_LINE_NUMS_STRIPPED deprecated, should be 0? */
        /* 0x0008 - IMAGE_FILE_LOCAL_SYMS_STRIPPED  deprecated, should be 0? */
        /* 0x0200 - IMAGE_FILE_DEBUG_STRIPPED */

optional_header:
    .short 0x020b /* Magic: PE32+ */
    .byte 0x02 /* MajorLinkerVersion */
    .byte 0x14 /* MinorLinkerVersion */
    .long _etext - _start /* SizeOfCode */
    .long _size_of_data /* SizeOfInitializedData */
    .long 0 /* SizeOfUninitializedData */
    .long _start - ImageBase /* AddressOfEntryPoint */
    .long _start - ImageBase /* BaseOfCode */
    .quad 0 /* ImageBase */
    .long 16 /* SectionAlignment */
    .long 16 /* FileAlignment */

    .short 0 /* MajorOperatingSystemVersion */
    .short 0 /* MinorOperatingSystemVersion */
    .short 0 /* MajorImageVersion */
    .short 0 /* MinorImageVersion */
    .short 0 /* MajorSubsystemVersion */
    .short 0 /* MinorSubsystemVersion */
    .long 0 /* Win32VersionValue */

    .long _edata - ImageBase /* SizeOfImage */
    .long _start - ImageBase /* SizeOfHeaders */
    .long 0 /* CheckSum */
    .short 10 /* Subsystem (EFI Application) */
    .short 0 /* DllCharacteristics */
    .long 0 /* SizeOfStackReserve */
    .long 0 /* SizeOfStackCommit */
    .long 0 /* SizeOfHeapReserve */
    .long 0 /* SizeOfHeapCommit */
    .long 0 /* LoaderFlags */
    .long 10 /* NumberOfRvaAndSizes */

    /* Data Directories (RVA & size) */
    .quad 0 /* export table */
    .quad 0 /* import table */
    .quad 0 /* resource table */
    .quad 0 /* exception table */
    .quad 0 /* attribute certificate table */
    .quad 0 /* base relocation table */
    .quad 0 /* debug data */
    .quad 0 /* architecture */
    .quad 0 /* GlobalPtr */
    .quad 0 /* thread local storage */

section_table:
    /* Dummy relocation section for EFI compliance */
    /* name (8 bytes) */
    .ascii ".reloc"
    .byte 0 /* name (7/8) */
    .byte 0 /* name (8/8) */
    .long 0 /* VirtualSize */
    .long 0 /* VirtualAddress */
    .long 0 /* SizeOfRawData */
    .long 0 /* PointerToRawData */
    .long 0 /* PointerToRelocators */
    .long 0 /* PointerToLinenumbers */
    .short 0 /* NumberOfRelocations */
    .short 0 /* NumberOfLinenumbers */
    .long 0x42100040 /* Characteristics */
        /* 0x00000040 - IMAGE_SCN_CNT_INITIALIZED_DATA */
        /* 0x00100000 - IMAGE_SCN_ALIGN_1BYTES */
        /* 0x02000000 - IMAGE_SCN_MEM_DISCARDABLE */
        /* 0x40000000 - IMAGE_SCN_MEM_READ */

    .ascii ".text"
    .byte 0 /* name (6/8) */
    .byte 0 /* name (7/8) */
    .byte 0 /* name (8/8) */
    .long _etext - _start /* VirtualSize */
    .long _start - ImageBase /* VirtualAddress */
    .long _etext - _start /* SizeOfRawData */
    .long _start - ImageBase /* PointerToRawData */
    .long 0 /* PointerToRelocators */
    .long 0 /* PointerToLinenumbers */
    .short 0 /* NumberOfRelocations */
    .short 0 /* NumberOfLinenumbers */
    .long 0xe0500020 /* Characteristics */
        /* 0x00000020 - IMAGE_SCN_CNT_CODE */
        /* 0x00500000 - IMAGE_SCN_ALIGN_16BYTES */
        /* 0x20000000 - IMAGE_SCN_MEM_EXECUTE */
        /* 0x40000000 - IMAGE_SCN_MEM_READ */
        /* 0x80000000 - IMAGE_SCN_MEM_WRITE */

    .ascii ".data"
    .byte 0 /* name (6/8) */
    .byte 0 /* name (7/8) */
    .byte 0 /* name (8/8) */
    .long _size_of_data /* VirtualSize */
    .long _data - ImageBase /* VirtualAddress */
    .long _size_of_data /* SizeOfRawData */
    .long _data - ImageBase /* PointerToRawData */
    .long 0 /* PointerToRelocators */
    .long 0 /* PointerToLinenumbers */
    .short 0 /* NumberOfRelocations */
    .short 0 /* NumberOfLinenumbers */
    .long 0xc0500040 /* Characteristics */
        /* 0x00000040 - IMAGE_SCN_CNT_INITIALIZED_DATA */
        /* 0x00500000 - IMAGE_SCN_ALIGN_16BYTES */
        /* 0x40000000 - IMAGE_SCN_MEM_READ */
        /* 0x80000000 - IMAGE_SCN_MEM_WRITE */

    .globl _start
_start:
    addi sp, sp, -32
    sd a0, 0(sp)
    sd a1, 8(sp)
    sd ra, 16(sp)

    lla a0, ImageBase
     lla a1, _DYNAMIC
    call relocate
    bnez a0, 0f

    ld a1, 8(sp)
    ld a0, 0(sp)
    call efi_main

0:
    ld ra, 16(sp)
    addi sp, sp, 32
    ret
