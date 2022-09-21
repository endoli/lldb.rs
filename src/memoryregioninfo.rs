use crate::{lldb_addr_t, sys, SBStream};
use std::ffi::CStr;
use std::fmt;

#[allow(missing_docs)]
pub struct SBMemoryRegionInfo {
    pub raw: sys::SBMemoryRegionInfoRef,
}

impl SBMemoryRegionInfo {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        SBMemoryRegionInfo::from(unsafe { sys::CreateSBMemoryRegionInfo() })
    }

    #[allow(missing_docs)]
    pub fn clear(&self) {
        unsafe { sys::SBMemoryRegionInfoClear(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn is_executable(&self) -> bool {
        unsafe { sys::SBMemoryRegionInfoIsExecutable(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_mapped(&self) -> bool {
        unsafe { sys::SBMemoryRegionInfoIsMapped(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_readable(&self) -> bool {
        unsafe { sys::SBMemoryRegionInfoIsReadable(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_writable(&self) -> bool {
        unsafe { sys::SBMemoryRegionInfoIsWritable(self.raw) }
    }

    #[allow(missing_docs)]
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

    #[allow(missing_docs)]
    pub fn get_region_base(&self) -> lldb_addr_t {
        unsafe { sys::SBMemoryRegionInfoGetRegionBase(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn get_region_end(&self) -> lldb_addr_t {
        unsafe { sys::SBMemoryRegionInfoGetRegionEnd(self.raw) }
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

impl From<sys::SBMemoryRegionInfoRef> for SBMemoryRegionInfo {
    fn from(raw: sys::SBMemoryRegionInfoRef) -> Self {
        Self { raw }
    }
}
