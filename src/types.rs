// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::stream::SBStream;
use super::{BasicType, DescriptionLevel, TypeClass};
use std::ffi::CStr;
use std::fmt;
use sys;

#[allow(missing_docs)]
pub struct SBType {
    /// The underlying raw `SBTypeRef`.
    pub raw: sys::SBTypeRef,
}

impl SBType {
    /// Construct a new `Some(SBType)` or `None`.
    pub fn maybe_wrap(raw: sys::SBTypeRef) -> Option<SBType> {
        if unsafe { sys::SBTypeIsValid(raw) } {
            Some(SBType { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBType` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBTypeIsValid(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_pointer_type(&self) -> bool {
        unsafe { sys::SBTypeIsPointerType(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_reference_type(&self) -> bool {
        unsafe { sys::SBTypeIsReferenceType(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_function_type(&self) -> bool {
        unsafe { sys::SBTypeIsFunctionType(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_polymorphic_class(&self) -> bool {
        unsafe { sys::SBTypeIsPolymorphicClass(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_array_type(&self) -> bool {
        unsafe { sys::SBTypeIsArrayType(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_vector_type(&self) -> bool {
        unsafe { sys::SBTypeIsVectorType(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_typedef_type(&self) -> bool {
        unsafe { sys::SBTypeIsTypedefType(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn pointer_type(&self) -> Option<SBType> {
        SBType::maybe_wrap(unsafe { sys::SBTypeGetPointerType(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn pointee_type(&self) -> Option<SBType> {
        SBType::maybe_wrap(unsafe { sys::SBTypeGetPointeeType(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn reference_type(&self) -> Option<SBType> {
        SBType::maybe_wrap(unsafe { sys::SBTypeGetReferenceType(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn typedefed_type(&self) -> Option<SBType> {
        SBType::maybe_wrap(unsafe { sys::SBTypeGetTypedefedType(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn dereferenced_type(&self) -> Option<SBType> {
        SBType::maybe_wrap(unsafe { sys::SBTypeGetDereferencedType(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn unqualified_type(&self) -> Option<SBType> {
        SBType::maybe_wrap(unsafe { sys::SBTypeGetUnqualifiedType(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn array_element_type(&self) -> Option<SBType> {
        SBType::maybe_wrap(unsafe { sys::SBTypeGetArrayElementType(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn vector_element_type(&self) -> Option<SBType> {
        SBType::maybe_wrap(unsafe { sys::SBTypeGetVectorElementType(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn canonical_type(&self) -> Option<SBType> {
        SBType::maybe_wrap(unsafe { sys::SBTypeGetCanonicalType(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn basic_type(&self) -> BasicType {
        unsafe { sys::SBTypeGetBasicType(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBTypeGetName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn display_type_name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBTypeGetDisplayTypeName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn type_class(&self) -> TypeClass {
        TypeClass::from_bits_truncate(unsafe { sys::SBTypeGetTypeClass(self.raw) })
    }
}

impl Clone for SBType {
    fn clone(&self) -> SBType {
        SBType {
            raw: unsafe { sys::CloneSBType(self.raw) },
        }
    }
}

impl fmt::Debug for SBType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBTypeGetDescription(self.raw, stream.raw, DescriptionLevel::Brief) };
        write!(fmt, "SBType {{ {} }}", stream.data())
    }
}

impl Drop for SBType {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBType(self.raw) };
    }
}

impl From<sys::SBTypeRef> for SBType {
    fn from(raw: sys::SBTypeRef) -> SBType {
        SBType { raw }
    }
}

unsafe impl Send for SBType {}
unsafe impl Sync for SBType {}

#[cfg(feature = "graphql")]
graphql_object!(SBType: super::debugger::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }

    field is_pointer_type() -> bool {
        self.is_pointer_type()
    }

    field is_reference_type() -> bool {
        self.is_reference_type()
    }

    field is_function_type() -> bool {
        self.is_function_type()
    }

    field is_polymorphic_class() -> bool {
        self.is_polymorphic_class()
    }

    field is_array_type() -> bool {
        self.is_array_type()
    }

    field is_vector_type() -> bool {
        self.is_vector_type()
    }

    field is_typedef_type() -> bool {
        self.is_typedef_type()
    }

    field pointer_type() -> Option<SBType> {
        self.pointer_type()
    }

    field pointee_type() -> Option<SBType> {
        self.pointee_type()
    }

    field reference_type() -> Option<SBType> {
        self.reference_type()
    }

    field typedefed_type() -> Option<SBType> {
        self.typedefed_type()
    }

    field dereferenced_type() -> Option<SBType> {
        self.dereferenced_type()
    }

    field unqualified_type() -> Option<SBType> {
        self.unqualified_type()
    }

    field array_element_type() -> Option<SBType> {
        self.array_element_type()
    }

    field vector_element_type() -> Option<SBType> {
        self.vector_element_type()
    }

    field canonical_type() -> Option<SBType> {
        self.canonical_type()
    }

    // TODO(bm) bind `basic_type`.

    field name() -> &str {
        self.name()
    }
});
