#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod cxx_auto_artifact_info;
mod cxx_auto_entry;
mod error;
mod ffi {
    pub(crate) mod ctypes;
}
mod gen {
    pub(crate) mod ctypes;
}
mod processing;

#[cfg(feature = "alloc")]
pub use crate::{cxx_auto_artifact_info::CxxAutoArtifactInfo, cxx_auto_entry::CxxAutoEntry, error::*};
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
pub fn process_artifacts(
    project_dir: &std::path::Path,
    out_dir: &std::path::Path,
    cfg_dir: &std::path::Path,
) -> BoxResult<()> {
    let out_dir = &out_dir.join("src");
    crate::processing::process_src_auto_module(project_dir, out_dir, cfg_dir)?;
    Ok(())
}
