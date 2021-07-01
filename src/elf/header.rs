static MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];

#[repr(packed)]
pub struct Header {
    identity: IdentityHeader,
    _type: u16,
    machine: u16,
    _version: u32,
    entry: u64,
    phoff: u64,
    _shoff: u64,
    _flags: u32,
    _ehsize: u16,
    phentsize: u16,
    phnum: u16,
    _shentsize: u16,
    _shnum: u16,
    _shstrndx: u16,
}

#[repr(packed)]
struct IdentityHeader {
    magic: [u8; 4],
    class: u8,
    _endian: u8,
    _version: u8,
    _abi: u8,
    _padding: u64,
}

impl Header {
    pub fn valid_magic(&self) -> bool {
        self.identity.magic == MAGIC
    }

    pub fn get_class(&self) -> Option<ElfClass> {
        core::convert::TryInto::<ElfClass>::try_into(self.identity.class).ok()
    }

    pub fn get_machine(&self) -> Option<ElfMachine> {
        core::convert::TryInto::<ElfMachine>::try_into(self.machine).ok()
    }

    pub fn get_entry_point(&self) -> usize {
        self.entry as usize
    }

    pub fn get_program_header_info(&self) -> TableInfo {
        TableInfo {
            offset: self.phoff as usize,
            entry_size: self.phentsize as usize,
            entry_count: self.phnum as usize,
        }
    }
}

#[derive(PartialEq)]
pub enum ElfClass {
    Elf32,
    Elf64,
}

impl core::convert::TryFrom<u8> for ElfClass {
    type Error = u8;

    fn try_from(raw: u8) -> Result<ElfClass, u8> {
        match raw {
            1 => Ok(ElfClass::Elf32),
            2 => Ok(ElfClass::Elf64),
            other => Err(other),
        }
    }
}

#[derive(PartialEq)]
pub enum ElfMachine {
    RiscV,
}

impl core::convert::TryFrom<u16> for ElfMachine {
    type Error = u16;

    fn try_from(raw: u16) -> Result<ElfMachine, u16> {
        match raw {
            0xf3 => Ok(ElfMachine::RiscV),
            other => Err(other),
        }
    }
}

pub struct TableInfo {
    pub offset: usize,
    pub entry_size: usize,
    pub entry_count: usize,
}
