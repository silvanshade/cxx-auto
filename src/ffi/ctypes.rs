#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_char {
    pub(crate) value: ::core::ffi::c_char,
}

impl ::core::fmt::Debug for c_char {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let value = self.value as u8 as ::core::primitive::char;
        ::core::fmt::Debug::fmt(&value, f)
    }
}

impl ::core::fmt::Display for c_char {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let value = self.value as u8 as ::core::primitive::char;
        ::core::fmt::Display::fmt(&value, f)
    }
}

unsafe impl cxx::ExternType for c_char {
    type Id = cxx::type_id!("c_char");
    type Kind = cxx::kind::Trivial;
}

impl From<::core::ffi::c_char> for c_char {
    #[inline]
    fn from(value: ::core::ffi::c_char) -> Self {
        c_char { value }
    }
}

impl From<c_char> for ::core::ffi::c_char {
    #[inline]
    fn from(wrapper: c_char) -> Self {
        wrapper.value
    }
}

impl From<::core::primitive::char> for c_char {
    #[inline]
    fn from(value: ::core::primitive::char) -> Self {
        c_char::from(value as ::core::ffi::c_char)
    }
}

impl From<c_char> for ::core::primitive::char {
    #[inline]
    fn from(wrapper: c_char) -> Self {
        wrapper.value as u8 as ::core::primitive::char
    }
}

impl crate::ffi::ctypes::c_char {
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> &[c_char] {
        let data = bytes.as_ptr().cast::<c_char>();
        let len = bytes.len();
        unsafe { ::core::slice::from_raw_parts(data, len) }
    }

    #[inline]
    pub fn into_bytes(slice: &[c_char]) -> &[u8] {
        let data = slice.as_ptr().cast::<u8>();
        let len = slice.len();
        unsafe { ::core::slice::from_raw_parts(data, len) }
    }

    #[cfg(feature = "std")]
    #[inline]
    pub fn from_path(path: &std::path::Path) -> &[c_char] {
        use std::os::unix::ffi::OsStrExt;
        let bytes = path.as_os_str().as_bytes();
        Self::from_bytes(bytes)
    }

    #[inline]
    pub fn from_str(str: &str) -> &[c_char] {
        Self::from_bytes(str.as_bytes())
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_int {
    pub(crate) value: ::core::ffi::c_int,
}

impl ::core::fmt::Display for c_int {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_long {
    pub(crate) value: ::core::ffi::c_long,
}

impl ::core::fmt::Display for c_long {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_longlong {
    pub(crate) value: ::core::ffi::c_longlong,
}

impl ::core::fmt::Display for c_longlong {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_schar(::core::ffi::c_schar);

unsafe impl cxx::ExternType for c_schar {
    type Id = cxx::type_id!("c_schar");
    type Kind = cxx::kind::Trivial;
}

impl From<c_schar> for ::core::ffi::c_schar {
    #[inline]
    fn from(value: c_schar) -> Self {
        value.0
    }
}

impl From<::core::ffi::c_schar> for c_schar {
    #[inline]
    fn from(value: ::core::ffi::c_schar) -> Self {
        c_schar(value)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_short {
    pub(crate) value: ::core::ffi::c_short,
}

impl ::core::fmt::Display for c_short {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_uchar {
    pub(crate) value: ::core::ffi::c_uchar,
}

impl ::core::fmt::Display for c_uchar {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_uint {
    pub(crate) value: ::core::ffi::c_uint,
}

impl ::core::fmt::Display for c_uint {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_ulong {
    pub(crate) value: ::core::ffi::c_ulong,
}

impl ::core::fmt::Display for c_ulong {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_ulonglong {
    pub(crate) value: ::core::ffi::c_ulonglong,
}

impl ::core::fmt::Display for c_ulonglong {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_ushort {
    pub(crate) value: ::core::ffi::c_ushort,
}

impl ::core::fmt::Display for c_ushort {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_void(::core::ffi::c_void);

impl From<::core::ffi::c_void> for c_void {
    #[inline]
    fn from(value: ::core::ffi::c_void) -> Self {
        c_void(value)
    }
}

impl From<c_void> for ::core::ffi::c_void {
    #[inline]
    fn from(value: c_void) -> Self {
        value.0
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_off_t {
    pub(crate) value: libc::off_t,
}

impl ::core::fmt::Debug for c_off_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.value, f)
    }
}

impl ::core::fmt::Display for c_off_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

unsafe impl cxx::ExternType for c_off_t {
    type Id = cxx::type_id!("c_off_t");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_time_t {
    pub(crate) value: libc::time_t,
}

impl ::core::fmt::Debug for c_time_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.value, f)
    }
}

impl ::core::fmt::Display for c_time_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

unsafe impl cxx::ExternType for c_time_t {
    type Id = cxx::type_id!("c_time_t");
    type Kind = cxx::kind::Trivial;
}
