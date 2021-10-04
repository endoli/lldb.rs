// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    lldb_addr_t, sys, SBBlock, SBCompileUnit, SBFunction, SBLineEntry, SBModule, SBSection,
    SBStream, SBSymbol, SBSymbolContext, SBTarget,
};
use std::fmt;

/// A section + offset based address class.
///
/// The `SBAddress` class allows addresses to be relative to a section
/// that can move during runtime due to images (executables, shared
/// libraries, bundles, frameworks) being loaded at different
/// addresses than the addresses found in the object file that
/// represents them on disk. There are currently two types of addresses
/// for a section:
///
/// * file addresses
/// * load addresses
///
/// File addresses represents the virtual addresses that are in the 'on
/// disk' object files. These virtual addresses are converted to be
/// relative to unique sections scoped to the object file so that
/// when/if the addresses slide when the images are loaded/unloaded
/// in memory, we can easily track these changes without having to
/// update every object (compile unit ranges, line tables, function
/// address ranges, lexical block and inlined subroutine address
/// ranges, global and static variables) each time an image is loaded or
/// unloaded.
///
/// Load addresses represents the virtual addresses where each section
/// ends up getting loaded at runtime. Before executing a program, it
/// is common for all of the load addresses to be unresolved. When a
/// `DynamicLoader` plug-in receives notification that shared libraries
/// have been loaded/unloaded, the load addresses of the main executable
/// and any images (shared libraries) will be  resolved/unresolved. When
/// this happens, breakpoints that are in one of these sections can be
/// set/cleared.
pub struct SBAddress {
    /// The underlying raw `SBAddressRef`.
    pub raw: sys::SBAddressRef,
}

impl SBAddress {
    /// Construct a new `Some(SBAddress)` or `None`.
    pub fn maybe_wrap(raw: sys::SBAddressRef) -> Option<SBAddress> {
        if unsafe { sys::SBAddressIsValid(raw) } {
            Some(SBAddress { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBAddress` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBAddressIsValid(self.raw) }
    }

    /// Construct a new `SBAddress` from the given section and offset.
    pub fn from_section_offset(section: &SBSection, offset: lldb_addr_t) -> SBAddress {
        let a = unsafe { sys::CreateSBAddress2(section.raw, offset) };
        SBAddress::from(a)
    }

    /// Create a new `SBAddress` from the given load address within the target.
    pub fn from_load_address(load_addr: lldb_addr_t, target: &SBTarget) -> SBAddress {
        let a = unsafe { sys::CreateSBAddress3(load_addr, target.raw) };
        SBAddress::from(a)
    }

    /// The address that represents the address as it is found in the
    /// object file that defines it.
    pub fn file_address(&self) -> u64 {
        unsafe { sys::SBAddressGetFileAddress(self.raw) }
    }

    /// The address as it has been loaded into memory by a target.
    pub fn load_address(&self, target: &SBTarget) -> u64 {
        unsafe { sys::SBAddressGetLoadAddress(self.raw, target.raw) }
    }

    /// Get the `SBSymbolContext` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// [`SBAddress::from_load_address`] or [`SBTarget::resolve_load_address`].
    ///
    /// * `resolve_scope`: Flags that specify what type of symbol context
    ///   is needed by the caller. These flags have constants starting
    ///   with `SYMBOL_CONTEXT_ITEM_`.
    pub fn symbol_context(&self, resolve_scope: u32) -> SBSymbolContext {
        SBSymbolContext::from(unsafe { sys::SBAddressGetSymbolContext(self.raw, resolve_scope) })
    }

    /// Get the `SBModule` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// [`SBAddress::from_load_address`] or [`SBTarget::resolve_load_address`].
    ///
    /// This grabs an individual object for a given address and
    /// is less efficient if you want more than one symbol related objects.
    /// Use one of the following when you want multiple debug symbol related
    /// objects for an address:
    ///
    /// * [`SBAddress::symbol_context`]
    /// * `SBTarget::resolve_symbol_context_for_address`
    ///
    /// One or more bits from the `SymbolContextItem` enumerations can be logically
    /// OR'ed together to more efficiently retrieve multiple symbol objects.
    pub fn module(&self) -> Option<SBModule> {
        SBModule::maybe_wrap(unsafe { sys::SBAddressGetModule(self.raw) })
    }

    /// Get the `SBCompileUnit` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// [`SBAddress::from_load_address`] or [`SBTarget::resolve_load_address`].
    ///
    /// This grabs an individual object for a given address and
    /// is less efficient if you want more than one symbol related objects.
    /// Use one of the following when you want multiple debug symbol related
    /// objects for an address:
    ///
    /// * [`SBAddress::symbol_context`]
    /// * `SBTarget::resolve_symbol_context_for_address`
    ///
    /// One or more bits from the `SymbolContextItem` enumerations can be logically
    /// OR'ed together to more efficiently retrieve multiple symbol objects.
    pub fn compile_unit(&self) -> Option<SBCompileUnit> {
        SBCompileUnit::maybe_wrap(unsafe { sys::SBAddressGetCompileUnit(self.raw) })
    }

    /// Get the `SBFunction` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// [`SBAddress::from_load_address`] or [`SBTarget::resolve_load_address`].
    ///
    /// This grabs an individual object for a given address and
    /// is less efficient if you want more than one symbol related objects.
    /// Use one of the following when you want multiple debug symbol related
    /// objects for an address:
    ///
    /// * [`SBAddress::symbol_context`]
    /// * `SBTarget::resolve_symbol_context_for_address`
    ///
    /// One or more bits from the `SymbolContextItem` enumerations can be logically
    /// OR'ed together to more efficiently retrieve multiple symbol objects.
    pub fn function(&self) -> Option<SBFunction> {
        SBFunction::maybe_wrap(unsafe { sys::SBAddressGetFunction(self.raw) })
    }

    /// Get the `SBBlock` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// [`SBAddress::from_load_address`] or [`SBTarget::resolve_load_address`].
    ///
    /// This grabs an individual object for a given address and
    /// is less efficient if you want more than one symbol related objects.
    /// Use one of the following when you want multiple debug symbol related
    /// objects for an address:
    ///
    /// * [`SBAddress::symbol_context`]
    /// * `SBTarget::resolve_symbol_context_for_address`
    ///
    /// One or more bits from the `SymbolContextItem` enumerations can be logically
    /// OR'ed together to more efficiently retrieve multiple symbol objects.
    pub fn block(&self) -> Option<SBBlock> {
        SBBlock::maybe_wrap(unsafe { sys::SBAddressGetBlock(self.raw) })
    }

    /// Get the `SBSymbol` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// [`SBAddress::from_load_address`] or [`SBTarget::resolve_load_address`].
    ///
    /// This grabs an individual object for a given address and
    /// is less efficient if you want more than one symbol related objects.
    /// Use one of the following when you want multiple debug symbol related
    /// objects for an address:
    ///
    /// * [`SBAddress::symbol_context`]
    /// * `SBTarget::resolve_symbol_context_for_address`
    ///
    /// One or more bits from the `SymbolContextItem` enumerations can be logically
    /// OR'ed together to more efficiently retrieve multiple symbol objects.
    pub fn symbol(&self) -> Option<SBSymbol> {
        SBSymbol::maybe_wrap(unsafe { sys::SBAddressGetSymbol(self.raw) })
    }

    /// Get the `SBLineEntry` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// [`SBAddress::from_load_address`] or [`SBTarget::resolve_load_address`].
    ///
    /// This grabs an individual object for a given address and
    /// is less efficient if you want more than one symbol related objects.
    /// Use one of the following when you want multiple debug symbol related
    /// objects for an address:
    ///
    /// * [`SBAddress::symbol_context`]
    /// * `SBTarget::resolve_symbol_context_for_address`
    ///
    /// One or more bits from the `SymbolContextItem` enumerations can be logically
    /// OR'ed together to more efficiently retrieve multiple symbol objects.
    pub fn line_entry(&self) -> Option<SBLineEntry> {
        SBLineEntry::maybe_wrap(unsafe { sys::SBAddressGetLineEntry(self.raw) })
    }
}

impl Clone for SBAddress {
    fn clone(&self) -> SBAddress {
        SBAddress {
            raw: unsafe { sys::CloneSBAddress(self.raw) },
        }
    }
}

impl fmt::Debug for SBAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBAddressGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBAddress {{ {} }}", stream.data())
    }
}

impl Drop for SBAddress {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBAddress(self.raw) };
    }
}

impl From<sys::SBAddressRef> for SBAddress {
    fn from(raw: sys::SBAddressRef) -> SBAddress {
        SBAddress { raw }
    }
}

unsafe impl Send for SBAddress {}
unsafe impl Sync for SBAddress {}

impl PartialEq for SBAddress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::SBAddressIsEqual(self.raw, other.raw) }
    }
}

impl Eq for SBAddress {}

#[cfg(feature = "graphql")]
graphql_object!(SBAddress: crate::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }

    // TODO(bm) This should be u64
    field file_address() -> i32 {
        self.file_address() as i32
    }

    field module() -> Option<SBModule> {
        self.module()
    }

    field compile_unit() -> Option<SBCompileUnit> {
        self.compile_unit()
    }

    field function() -> Option<SBFunction> {
        self.function()
    }

    field block() -> Option<SBBlock> {
        self.block()
    }

    field symbol() -> Option<SBSymbol> {
        self.symbol()
    }

    field line_entry() -> Option<SBLineEntry> {
        self.line_entry()
    }
});

#[cfg(test)]
mod tests {
    use super::SBAddress;

    #[test]
    fn test_equal() {
        let sect = unsafe { sys::CreateSBSection() };
        let a = SBAddress::maybe_wrap(unsafe { sys::CreateSBAddress2(sect, 42) }).unwrap();
        let b = SBAddress::maybe_wrap(unsafe { sys::CreateSBAddress2(sect, 42) }).unwrap();
        assert!(a == b);
    }

    #[test]
    fn test_not_equal() {
        let sect = unsafe { sys::CreateSBSection() };
        let a = SBAddress::maybe_wrap(unsafe { sys::CreateSBAddress2(sect, 42) }).unwrap();
        let b = SBAddress::maybe_wrap(unsafe { sys::CreateSBAddress2(sect, 111) }).unwrap();
        assert!(a != b);
    }
}
