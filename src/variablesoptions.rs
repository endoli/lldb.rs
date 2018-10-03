// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::DynamicValueType;
use sys;

#[allow(missing_docs)]
#[derive(Debug)]
pub struct SBVariablesOptions {
    /// The underlying raw `SBVariablesOptionsRef`.
    pub raw: sys::SBVariablesOptionsRef,
}

impl SBVariablesOptions {
    /// Construct a new `SBVariablesOptions`.
    pub fn new() -> SBVariablesOptions {
        SBVariablesOptions::wrap(unsafe { sys::CreateSBVariablesOptions() })
    }

    /// Construct a new `SBVariablesOptions`.
    pub fn wrap(raw: sys::SBVariablesOptionsRef) -> SBVariablesOptions {
        SBVariablesOptions { raw }
    }

    /// Construct a new `Some(SBVariablesOptions)` or `None`.
    pub fn maybe_wrap(raw: sys::SBVariablesOptionsRef) -> Option<SBVariablesOptions> {
        if unsafe { sys::SBVariablesOptionsIsValid(raw) != 0 } {
            Some(SBVariablesOptions { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBVariablesOptions` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBVariablesOptionsIsValid(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn include_arguments(&self) -> bool {
        unsafe { sys::SBVariablesOptionsGetIncludeArguments(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn set_include_arguments(&self, arguments: bool) {
        unsafe { sys::SBVariablesOptionsSetIncludeArguments(self.raw, arguments as u8) };
    }

    #[allow(missing_docs)]
    pub fn include_locals(&self) -> bool {
        unsafe { sys::SBVariablesOptionsGetIncludeLocals(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn set_include_locals(&self, locals: bool) {
        unsafe { sys::SBVariablesOptionsSetIncludeLocals(self.raw, locals as u8) };
    }

    #[allow(missing_docs)]
    pub fn include_statics(&self) -> bool {
        unsafe { sys::SBVariablesOptionsGetIncludeStatics(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn set_include_statics(&self, statics: bool) {
        unsafe { sys::SBVariablesOptionsSetIncludeStatics(self.raw, statics as u8) };
    }

    #[allow(missing_docs)]
    pub fn in_scope_only(&self) -> bool {
        unsafe { sys::SBVariablesOptionsGetInScopeOnly(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn set_in_scope_only(&self, in_scope_only: bool) {
        unsafe { sys::SBVariablesOptionsSetInScopeOnly(self.raw, in_scope_only as u8) };
    }

    #[allow(missing_docs)]
    pub fn include_runtime_support_values(&self) -> bool {
        unsafe { sys::SBVariablesOptionsGetIncludeRuntimeSupportValues(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn set_include_runtime_support_values(&self, include: bool) {
        unsafe { sys::SBVariablesOptionsSetIncludeRuntimeSupportValues(self.raw, include as u8) };
    }

    #[allow(missing_docs)]
    pub fn use_dynamic(&self) -> DynamicValueType {
        unsafe { sys::SBVariablesOptionsGetUseDynamic(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_use_dynamic(&self, use_dynamic: DynamicValueType) {
        unsafe { sys::SBVariablesOptionsSetUseDynamic(self.raw, use_dynamic) };
    }
}

impl Clone for SBVariablesOptions {
    fn clone(&self) -> SBVariablesOptions {
        SBVariablesOptions {
            raw: unsafe { sys::CloneSBVariablesOptions(self.raw) },
        }
    }
}

impl Default for SBVariablesOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SBVariablesOptions {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBVariablesOptions(self.raw) };
    }
}

unsafe impl Send for SBVariablesOptions {}
unsafe impl Sync for SBVariablesOptions {}
