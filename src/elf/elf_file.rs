use super::util::raw_cast;
use super::{
    header,
    program_header::ProgramHeader,
    ElfError,
    Header,
};

pub struct ElfFile<'a> {
    raw_buffer: &'a [u8],
}

impl <'a> ElfFile<'a> {
    /// Unsafe: Appropriate memory alignment must be ensured by caller
    pub unsafe fn from_buffer(buffer: &'a [u8]) -> Result<ElfFile<'a>, ElfError> {
        if let Some(header) = raw_cast::<Header>(&buffer[..]) {
            if header.valid_magic() != true {
                return Err(ElfError::InvalidFormat);
            }

            if header.get_class() != Some(header::ElfClass::Elf64) {
                return Err(ElfError::IncompatibleMachine);
            }

            if header.get_machine() != Some(header::ElfMachine::RiscV) {
                return Err(ElfError::IncompatibleMachine);
            }

            Ok(ElfFile { raw_buffer: buffer })
        } else {
            Err(ElfError::InvalidFormat)
        }
    }

    pub fn header(&self) -> &Header {
        // `from_buffer` already checked that we have a valid Header,
        // so this should never fail.
        unsafe { raw_cast::<Header>(&self.raw_buffer[..]).unwrap() }
    }

    pub fn program_headers(&self) -> Result<ProgramHeaderIterator<'a>, ElfError> {
        ProgramHeaderIterator::new(
            self.raw_buffer,
            self.header().get_program_header_info()
        )
    }

    pub fn segment_data(
        &self,
        program_header: &ProgramHeader,
    ) -> Result<&'a [u8], ElfError> {
        let start = program_header.get_offset();
        let end = start + program_header.get_file_size();

        if end <= self.raw_buffer.len() {
            Ok(&self.raw_buffer[start..end])
        } else {
            Err(ElfError::BufferOverflow)
        }
    }
}

pub struct ProgramHeaderIterator<'a> {
    table_info: header::TableInfo,
    raw_buffer: &'a [u8],
    next_index: usize,
}

impl<'a> ProgramHeaderIterator<'a> {
    fn new(
        buffer: &'a [u8],
        table_info: header::TableInfo,
    ) -> Result<Self, ElfError> {
        // require 8 byte alignment
        if table_info.entry_size & 0x7 > 0 {
            return Err(ElfError::InvalidFormat);
        }

        // check buffer size
        let table_end = table_info.offset + table_info.entry_count * table_info.entry_size;
        if table_end > buffer.len() {
            return Err(ElfError::InvalidFormat);
        }

        Ok(ProgramHeaderIterator {
            table_info,
            raw_buffer: buffer,
            next_index: 0,
        })
    }
}

impl<'a> core::iter::Iterator for ProgramHeaderIterator<'a> {
    type Item = &'a ProgramHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.table_info.entry_count {
            return None;
        }

        let entry_size = self.table_info.entry_size;
        let offset = self.table_info.offset + (entry_size * self.next_index);
        self.next_index += 1;

        // Buffer size and entry alignment are checked by
        // `ProgramSegmentIterator::new`.
        unsafe {
            raw_cast::<ProgramHeader>(
                &self.raw_buffer[offset..(offset + entry_size)]
            )
        }
    }
}
