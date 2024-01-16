use crate::sys;
use crate::SBMemoryRegionInfo;

/// A list of [memory regions].
///
/// This is returned from
/// [`SBProcess::get_memory_regions()`](crate::SBProcess::get_memory_regions).
///
/// [memory regions]: SBMemoryRegionInfo
#[derive(Debug)]
pub struct SBMemoryRegionInfoList {
    /// The underlying raw `SBMemoryRegionInfoListRef`.
    pub raw: sys::SBMemoryRegionInfoListRef,
}

impl SBMemoryRegionInfoList {
    /// Construct a new `SBMemoryRegionInfoList`.
    pub(crate) fn wrap(raw: sys::SBMemoryRegionInfoListRef) -> SBMemoryRegionInfoList {
        SBMemoryRegionInfoList { raw }
    }

    #[allow(missing_docs)]
    pub fn append(&self, region: SBMemoryRegionInfo) {
        unsafe { sys::SBMemoryRegionInfoListAppend(self.raw, region.raw) };
    }

    #[allow(missing_docs)]
    pub fn append_list(&self, region_list: SBMemoryRegionInfoList) {
        unsafe { sys::SBMemoryRegionInfoListAppendList(self.raw, region_list.raw) };
    }

    /// Is this memory region list empty?
    pub fn is_empty(&self) -> bool {
        unsafe { sys::SBMemoryRegionInfoListGetSize(self.raw) == 0 }
    }

    /// Clear this memory region list.
    pub fn clear(&self) {
        unsafe { sys::SBMemoryRegionInfoListClear(self.raw) };
    }

    /// Iterate over this memory region list.
    pub fn iter(&self) -> SBMemoryRegionInfoListIter {
        SBMemoryRegionInfoListIter { list: self, idx: 0 }
    }
}

impl Clone for SBMemoryRegionInfoList {
    fn clone(&self) -> Self {
        Self {
            raw: unsafe { sys::CloneSBMemoryRegionInfoList(self.raw) },
        }
    }
}

impl Drop for SBMemoryRegionInfoList {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBMemoryRegionInfoList(self.raw) };
    }
}

impl<'d> IntoIterator for &'d SBMemoryRegionInfoList {
    type IntoIter = SBMemoryRegionInfoListIter<'d>;
    type Item = SBMemoryRegionInfo;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

unsafe impl Send for SBMemoryRegionInfoList {}
unsafe impl Sync for SBMemoryRegionInfoList {}

/// An iterator over the [memory regions] in an [`SBMemoryRegionInfoList`].
///
/// [memory regions]: SBMemoryRegionInfo
pub struct SBMemoryRegionInfoListIter<'d> {
    list: &'d SBMemoryRegionInfoList,
    idx: u32,
}

impl<'d> Iterator for SBMemoryRegionInfoListIter<'d> {
    type Item = SBMemoryRegionInfo;

    fn next(&mut self) -> Option<SBMemoryRegionInfo> {
        if self.idx < unsafe { sys::SBMemoryRegionInfoListGetSize(self.list.raw) } {
            let info = SBMemoryRegionInfo::default();
            let r = if unsafe {
                sys::SBMemoryRegionInfoListGetMemoryRegionAtIndex(self.list.raw, self.idx, info.raw)
            } {
                Some(info)
            } else {
                None
            };
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBMemoryRegionInfoListGetSize(self.list.raw) } as usize;
        (sz - self.idx as usize, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBMemoryRegionInfoListIter<'d> {}
