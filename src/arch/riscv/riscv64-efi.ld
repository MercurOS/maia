OUTPUT_FORMAT("elf64-littleriscv", "elf64-littleriscv", "elf64-littleriscv")
OUTPUT_ARCH(riscv)
ENTRY(_start)
SECTIONS
{
    .text 0x0 : {
        _text = .;
        *(.text.head)
        *(.text)
        *(.text.*)
        *(.gnu.linkonce.t.*)
        *(.srodata)
        *(.rodata*)
        . = ALIGN(16);
    }
    _etext = .;

    .dynamic : { *(.dynamic) }

    .data : {
        _data = .;
        *(.sdata)
        *(.data)
        *(.data1)
        *(.data.*)
        *(.got.plt)
        *(.got)

        . = ALIGN(16);
        _bss = .;
        *(.sbss)
        *(.scommon)
        *(.dynbss)
        *(.bss)
        *(.bss.*)
        *(COMMON)
        . = ALIGN(16);
        _ebss = .;
    }
    .rela.dyn : { *(.rela.dyn) }
    .rela.plt : { *(.rela.plt) }
    .rela.got : { *(.rela.got) }
    .rela.data : { *(.rela.data) *(.rela.data*) }
    _edata = .;

    . = ALIGN(4096);
    .dynsym : { *(.dynsym) }
    . = ALIGN(4096);
    .dynstr : { *(.dynstr) }
    . = ALIGN(4096);
    .note.gnu.build-id : { *(.note.gnu.build-id) }

    /DISCARD/ : {
        *(.rel.reloc)
        *(.eh_frame)
        *(.note.GNU-stack)
        *(.comment)
    }
}
