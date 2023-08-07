#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod cxx_abi_artifact_info;
mod cxx_abi_entry;
mod error;
mod ffi {
    pub(crate) mod ctypes;
}
mod gen {
    pub(crate) mod ctypes;
}
mod processing;

#[cfg(feature = "alloc")]
pub use crate::{cxx_abi_artifact_info::CxxAbiArtifactInfo, cxx_abi_entry::CxxAbiEntry, error::*};
#[cfg(feature = "alloc")]
pub use indexmap;

pub mod ctypes {
    pub use crate::ffi::ctypes::{
        c_char,
        c_int,
        c_long,
        c_longlong,
        c_off_t,
        c_schar,
        c_short,
        c_time_t,
        c_uchar,
        c_uint,
        c_ulong,
        c_ulonglong,
        c_ushort,
        c_void,
    };
}

#[cfg(feature = "std")]
pub fn process_artifacts(abi_dir: &std::path::Path, abi_crate_src_dir: &std::path::Path) -> BoxResult<()> {
    crate::processing::process_src_abi_module(abi_dir, abi_crate_src_dir)?;
    Ok(())
}
