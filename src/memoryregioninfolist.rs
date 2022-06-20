use crate::sys;
use crate::SBMemoryRegionInfo;

#[allow(missing_docs)]
#[derive(Debug)]
pub struct SBMemoryRegionInfoList {
    pub raw: sys::SBMemoryRegionInfoListRef,
}

impl SBMemoryRegionInfoList {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        SBMemoryRegionInfoList::from(unsafe { sys::CreateSBMemoryRegionInfoList() })
    }

    #[allow(missing_docs)]
    pub fn append(&self, region: SBMemoryRegionInfo) {
        unsafe { sys::SBMemoryRegionInfoListAppend(self.raw, region.raw) };
    }

    #[allow(missing_docs)]
    pub fn append_list(&self, region_list: SBMemoryRegionInfoList) {
        unsafe { sys::SBMemoryRegionInfoListAppendList(self.raw, region_list.raw) };
    }

    #[allow(missing_docs)]
    pub fn clear(&self) {
        unsafe { sys::SBMemoryRegionInfoListClear(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn get_memory_region(&self, index: u32) -> Option<SBMemoryRegionInfo> {
        let tmp = SBMemoryRegionInfo::default();
        if unsafe { sys::SBMemoryRegionInfoListGetMemoryRegionAtIndex(self.raw, index, tmp.raw) } {
            Some(tmp)
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn get_size(&self) -> u32 {
        unsafe { sys::SBMemoryRegionInfoListGetSize(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn iter(&self) -> SBMemoryRegionInfoListIter {
        SBMemoryRegionInfoListIter {
            memory_list: self,
            idx: 0,
        }
    }
}

impl Clone for SBMemoryRegionInfoList {
    fn clone(&self) -> Self {
        Self {
            raw: unsafe { sys::CloneSBMemoryRegionInfoList(self.raw) },
        }
    }
}

impl Default for SBMemoryRegionInfoList {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SBMemoryRegionInfoList {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBMemoryRegionInfoList(self.raw) };
    }
}

impl From<sys::SBMemoryRegionInfoListRef> for SBMemoryRegionInfoList {
    fn from(raw: sys::SBMemoryRegionInfoListRef) -> Self {
        Self { raw }
    }
}

#[allow(missing_docs)]
pub struct SBMemoryRegionInfoListIter<'d> {
    memory_list: &'d SBMemoryRegionInfoList,
    idx: u32,
}

impl<'d> Iterator for SBMemoryRegionInfoListIter<'d> {
    type Item = SBMemoryRegionInfo;

    #[allow(missing_docs)]
    fn next(&mut self) -> Option<SBMemoryRegionInfo> {
        if self.idx < self.memory_list.get_size() {
            let r = self.memory_list.get_memory_region(self.idx);
            self.idx += 1;
            r
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = self.memory_list.get_size() as usize;
        (sz - self.idx as usize, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBMemoryRegionInfoListIter<'d> {}
