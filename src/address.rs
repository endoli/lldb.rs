// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::block::SBBlock;
use super::compileunit::SBCompileUnit;
use super::function::SBFunction;
use super::lineentry::SBLineEntry;
use super::module::SBModule;
use super::symbol::SBSymbol;
use super::symbolcontext::SBSymbolContext;
use super::target::SBTarget;
use super::AddressClass;
use sys;

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
/// DynamicLoader plug-in receives notification that shared libraries
/// have been loaded/unloaded, the load addresses of the main executable
/// and any images (shared libraries) will be  resolved/unresolved. When
/// this happens, breakpoints that are in one of these sections can be
/// set/cleared.
#[derive(Debug)]
pub struct SBAddress {
    /// The underlying raw `SBAddressRef`.
    pub raw: sys::SBAddressRef,
}

impl SBAddress {
    /// Construct a new `SBAddress`.
    pub fn new(raw: sys::SBAddressRef) -> SBAddress {
        SBAddress { raw: raw }
    }

    /// Construct a new `Some(SBAddress)` or `None`.
    pub fn maybe(raw: sys::SBAddressRef) -> Option<SBAddress> {
        if unsafe { sys::SBAddressIsValid(raw) != 0 } {
            Some(SBAddress { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBAddress` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBAddressIsValid(self.raw) != 0 }
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

    /// Get the address class for this address.
    pub fn address_class(&self) -> AddressClass {
        unsafe { sys::SBAddressGetAddressClass(self.raw) }
    }

    /// Get the `SBSymbolContext` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// `SBAddress::set_load_address` or `SBTarget::resolve_load_address`.
    pub fn symbol_context(&self, resolve_scope: u32) -> SBSymbolContext {
        SBSymbolContext::new(unsafe { sys::SBAddressGetSymbolContext(self.raw, resolve_scope) })
    }

    /// Get the `SBModule` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// `SBAddress::set_load_address` or `SBTarget::resolve_load_address`.
    ///
    /// This grabs an individual object for a given address and
    /// is less efficient if you want more than one symbol related objects.
    /// Use one of the following when you want multiple debug symbol related
    /// objects for an address:
    ///
    /// * `SBAddress::symbol_context`
    /// * `SBTarget::resolve_symbol_context_for_address`
    ///
    /// One or more bits from the `SymbolContextItem` enumerations can be logically
    /// OR'ed together to more efficiently retrieve multiple symbol objects.
    pub fn module(&self) -> Option<SBModule> {
        SBModule::maybe(unsafe { sys::SBAddressGetModule(self.raw) })
    }

    /// Get the `SBCompileUnit` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// `SBAddress::set_load_address` or `SBTarget::resolve_load_address`.
    ///
    /// This grabs an individual object for a given address and
    /// is less efficient if you want more than one symbol related objects.
    /// Use one of the following when you want multiple debug symbol related
    /// objects for an address:
    ///
    /// * `SBAddress::symbol_context`
    /// * `SBTarget::resolve_symbol_context_for_address`
    ///
    /// One or more bits from the `SymbolContextItem` enumerations can be logically
    /// OR'ed together to more efficiently retrieve multiple symbol objects.
    pub fn compile_unit(&self) -> Option<SBCompileUnit> {
        SBCompileUnit::maybe(unsafe { sys::SBAddressGetCompileUnit(self.raw) })
    }

    /// Get the `SBFunction` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// `SBAddress::set_load_address` or `SBTarget::resolve_load_address`.
    ///
    /// This grabs an individual object for a given address and
    /// is less efficient if you want more than one symbol related objects.
    /// Use one of the following when you want multiple debug symbol related
    /// objects for an address:
    ///
    /// * `SBAddress::symbol_context`
    /// * `SBTarget::resolve_symbol_context_for_address`
    ///
    /// One or more bits from the `SymbolContextItem` enumerations can be logically
    /// OR'ed together to more efficiently retrieve multiple symbol objects.
    pub fn function(&self) -> Option<SBFunction> {
        SBFunction::maybe(unsafe { sys::SBAddressGetFunction(self.raw) })
    }

    /// Get the `SBBlock` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// `SBAddress::set_load_address` or `SBTarget::resolve_load_address`.
    ///
    /// This grabs an individual object for a given address and
    /// is less efficient if you want more than one symbol related objects.
    /// Use one of the following when you want multiple debug symbol related
    /// objects for an address:
    ///
    /// * `SBAddress::symbol_context`
    /// * `SBTarget::resolve_symbol_context_for_address`
    ///
    /// One or more bits from the `SymbolContextItem` enumerations can be logically
    /// OR'ed together to more efficiently retrieve multiple symbol objects.
    pub fn block(&self) -> Option<SBBlock> {
        SBBlock::maybe(unsafe { sys::SBAddressGetBlock(self.raw) })
    }

    /// Get the `SBSymbol` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// `SBAddress::set_load_address` or `SBTarget::resolve_load_address`.
    ///
    /// This grabs an individual object for a given address and
    /// is less efficient if you want more than one symbol related objects.
    /// Use one of the following when you want multiple debug symbol related
    /// objects for an address:
    ///
    /// * `SBAddress::symbol_context`
    /// * `SBTarget::resolve_symbol_context_for_address`
    ///
    /// One or more bits from the `SymbolContextItem` enumerations can be logically
    /// OR'ed together to more efficiently retrieve multiple symbol objects.
    pub fn symbol(&self) -> Option<SBSymbol> {
        SBSymbol::maybe(unsafe { sys::SBAddressGetSymbol(self.raw) })
    }

    /// Get the `SBLineEntry` for a given address.
    ///
    /// An address might refer to code or data from an existing
    /// module, or it might refer to something on the stack or heap.
    /// This will only return valid values if the address has been
    /// resolved to a code or data address using
    /// `SBAddress::set_load_address` or `SBTarget::resolve_load_address`.
    ///
    /// This grabs an individual object for a given address and
    /// is less efficient if you want more than one symbol related objects.
    /// Use one of the following when you want multiple debug symbol related
    /// objects for an address:
    ///
    /// * `SBAddress::symbol_context`
    /// * `SBTarget::resolve_symbol_context_for_address`
    ///
    /// One or more bits from the `SymbolContextItem` enumerations can be logically
    /// OR'ed together to more efficiently retrieve multiple symbol objects.
    pub fn line_entry(&self) -> Option<SBLineEntry> {
        SBLineEntry::maybe(unsafe { sys::SBAddressGetLineEntry(self.raw) })
    }
}

impl Drop for SBAddress {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBAddress(self.raw) };
    }
}
