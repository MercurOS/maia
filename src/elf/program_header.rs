#[repr(packed)]
pub struct ProgramHeader {
    r#type: u32,
    _flags: u32,
    offset: u64,
    vaddr: u64,
    _paddr: u64,
    filesz: u64,
    memsz: u64,
    align: u64,
}

impl ProgramHeader {
    pub fn get_type(&self) -> Option<SegmentType> {
        core::convert::TryInto::<SegmentType>::try_into(self.r#type).ok()
    }

    pub fn get_offset(&self) -> usize {
        self.offset as usize
    }

    pub fn get_virtual_address(&self) -> usize {
        self.vaddr as usize
    }

    pub fn get_file_size(&self) -> usize {
        self.filesz as usize
    }

    pub fn get_memory_size(&self) -> usize {
        self.memsz as usize
    }

    pub fn get_alignment(&self) -> usize {
        self.align as usize
    }

    pub fn get_page_base(&self) -> usize {
        self.get_virtual_address() & !(self.get_alignment() - 1)
    }

    pub fn address_in_segment(&self, virtual_address: usize) -> bool {
        if virtual_address >= self.get_virtual_address() {
            if virtual_address < self.get_virtual_address() + self.get_memory_size() {
                return true;
            }
        }
        return false;
    }
}

#[derive(PartialEq)]
pub enum SegmentType {
    Load,
    Dynamic,
}

impl core::convert::TryFrom<u32> for SegmentType {
    type Error = u32;

    fn try_from(raw: u32) -> Result<SegmentType, u32> {
        match raw {
            1 => Ok(SegmentType::Load),
            2 => Ok(SegmentType::Dynamic),
            other => Err(other),
        }
    }
}
