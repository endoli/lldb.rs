use crate::{lldb_addr_t, sys, SBStream};
use std::ffi::CStr;
use std::fmt;

/// Information about memory regions within a process.
///
/// See also:
/// - [`SBProcess::get_memory_region_info()`](crate::SBProcess::get_memory_region_info)
/// - [`SBProcess::get_memory_regions()`](crate::SBProcess::get_memory_regions)
pub struct SBMemoryRegionInfo {
    /// The underlying raw `SBMemoryRegionInfoRef`.
    pub raw: sys::SBMemoryRegionInfoRef,
}

impl SBMemoryRegionInfo {
    #[allow(missing_docs)]
    pub(crate) fn new() -> Self {
        SBMemoryRegionInfo::wrap(unsafe { sys::CreateSBMemoryRegionInfo() })
    }

    /// Construct a new `SBMemoryRegionInfo`.
    pub(crate) fn wrap(raw: sys::SBMemoryRegionInfoRef) -> SBMemoryRegionInfo {
        SBMemoryRegionInfo { raw }
    }

    #[allow(missing_docs)]
    pub fn clear(&self) {
        unsafe { sys::SBMemoryRegionInfoClear(self.raw) };
    }

    /// Get the base address of this memory range.
    ///
    /// See also:
    ///
    /// - [SBMemoryRegionInfo::get_region_end()`]
    pub fn get_region_base(&self) -> lldb_addr_t {
        unsafe { sys::SBMemoryRegionInfoGetRegionBase(self.raw) }
    }

    /// Get the end address of this memory range.
    ///
    /// See also:
    ///
    /// - [SBMemoryRegionInfo::get_region_base()`]
    pub fn get_region_end(&self) -> lldb_addr_t {
        unsafe { sys::SBMemoryRegionInfoGetRegionEnd(self.raw) }
    }

    /// Check if this memory address is marked readable to the process.
    ///
    /// See also:
    ///
    /// - [SBMemoryRegionInfo::is_writable()`]
    /// - [SBMemoryRegionInfo::is_executable()`]
    pub fn is_readable(&self) -> bool {
        unsafe { sys::SBMemoryRegionInfoIsReadable(self.raw) }
    }

    /// Check if this memory address is marked writable to the process.
    ///
    /// See also:
    ///
    /// - [SBMemoryRegionInfo::is_readable()`]
    /// - [SBMemoryRegionInfo::is_executable()`]
    pub fn is_writable(&self) -> bool {
        unsafe { sys::SBMemoryRegionInfoIsWritable(self.raw) }
    }

    /// Check if this memory address is marked executable to the process.
    ///
    /// See also:
    ///
    /// - [SBMemoryRegionInfo::is_readable()`]
    /// - [SBMemoryRegionInfo::is_writable()`]
    pub fn is_executable(&self) -> bool {
        unsafe { sys::SBMemoryRegionInfoIsExecutable(self.raw) }
    }

    /// Check if this memory address is mapped into the process address
    /// space.
    pub fn is_mapped(&self) -> bool {
        unsafe { sys::SBMemoryRegionInfoIsMapped(self.raw) }
    }

    /// Returns the name of the memory region mapped at the given
    /// address.
    ///
    /// In case of memory mapped files it is the absolute path of
    /// the file, otherwise it is a name associated with the memory
    /// region. If no name can be determined, it returns `None`.
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            let ptr = sys::SBMemoryRegionInfoGetName(self.raw);

            if !ptr.is_null() {
                match CStr::from_ptr(ptr).to_str() {
                    Ok(s) => Some(s.to_owned()),
                    _ => panic!("No MemoryRegionInfo name string?"),
                }
            } else {
                None
            }
        }
    }

    /// Returns whether this memory region has a list of memory pages
    /// that have been modified -- that are dirty.
    ///
    /// The number of dirty pages may still be `0`.
    ///
    /// See also:
    ///
    /// - [`SBMemoryRegionInfo::dirty_pages()`]
    pub fn has_dirty_memory_page_list(&self) -> bool {
        unsafe { sys::SBMemoryRegionInfoHasDirtyMemoryPageList(self.raw) }
    }

    /// Returns an iterator over the addresses of modified pages in
    /// this region.
    ///
    /// Check [`SBMemoryRegionInfo::has_dirty_memory_page_list()`] to
    /// see if this information is available for this region.
    pub fn dirty_pages(&self) -> SBMemoryRegionInfoDirtyPageIter {
        SBMemoryRegionInfoDirtyPageIter { info: self, idx: 0 }
    }

    /// Returns the size of a memory page in this region
    /// or `0` if this information is unavailable.
    pub fn get_page_size(&self) -> i32 {
        unsafe { sys::SBMemoryRegionInfoGetPageSize(self.raw) }
    }
}

impl Clone for SBMemoryRegionInfo {
    fn clone(&self) -> Self {
        Self {
            raw: unsafe { sys::CloneSBMemoryRegionInfo(self.raw) },
        }
    }
}

impl fmt::Debug for SBMemoryRegionInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBMemoryRegionInfoGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBMemoryRegionInfo {{ {} }}", stream.data())
    }
}

impl Default for SBMemoryRegionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SBMemoryRegionInfo {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBMemoryRegionInfo(self.raw) };
    }
}

unsafe impl Send for SBMemoryRegionInfo {}
unsafe impl Sync for SBMemoryRegionInfo {}

/// Iterate over the addresses of dirty pages in a [memory region].
///
/// [memory region]: SBMemoryRegionInfo
pub struct SBMemoryRegionInfoDirtyPageIter<'d> {
    info: &'d SBMemoryRegionInfo,
    idx: u32,
}

impl<'d> Iterator for SBMemoryRegionInfoDirtyPageIter<'d> {
    type Item = lldb_addr_t;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < unsafe { sys::SBMemoryRegionInfoGetNumDirtyPages(self.info.raw) } {
            let r = Some(unsafe {
                sys::SBMemoryRegionInfoGetDirtyPageAddressAtIndex(self.info.raw, self.idx)
            });
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBMemoryRegionInfoGetNumDirtyPages(self.info.raw) } as usize;
        (sz - self.idx as usize, Some(sz))
    }
}

#[cfg(feature = "graphql")]
#[juniper::graphql_object]
impl SBMemoryRegionInfo {
    // TODO(bm) This should be u64
    fn region_base() -> i32 {
        self.get_region_base() as i32
    }

    // TODO(bm) This should be u64
    fn region_end() -> i32 {
        self.get_region_end() as i32
    }

    fn is_readable() -> bool {
        self.is_readable()
    }

    fn is_writable() -> bool {
        self.is_writable()
    }

    fn is_executable() -> bool {
        self.is_executable()
    }

    fn is_mapped() -> bool {
        self.is_mapped()
    }

    fn name() -> Option<String> {
        self.get_name()
    }

    fn page_size() -> i32 {
        self.get_page_size()
    }
}
