use crate::{error::AmdSmiError, types::Result};

#[macro_export]
macro_rules! amdsmi_unsafe {
    ($fn:expr) => {{
        let status = unsafe { $fn };
        if status != amdsmi_sys::amdsmi_status_t::AMDSMI_STATUS_SUCCESS {
            Err($crate::error::AmdSmiError::from(status))
        } else {
            Ok(())
        }
    }};
}

fn find_amdsmi_in_dir(dir: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut libs: Vec<_> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| {
            let p = e.ok()?.path();
            p.file_name()?
                .to_string_lossy()
                .starts_with("libamd_smi.so")
                .then_some(p)
        })
        .collect();

    libs.sort();
    libs.reverse();
    libs.into_iter().next()
}

pub fn find_amdsmi_path() -> Result<std::path::PathBuf> {
    if let Ok(dir) = std::env::var("AMDSMI_LIB_DIR") {
        return find_amdsmi_in_dir(std::path::Path::new(&dir)).ok_or(AmdSmiError::LibraryNotFound);
    }

    let mut candidates: Vec<_> = std::fs::read_dir("/opt")
        .into_iter()
        .flatten()
        .filter_map(|e| {
            let p = e.ok()?.path();
            p.file_name()?
                .to_string_lossy()
                .starts_with("rocm")
                .then_some(p.join("lib"))
        })
        .collect();

    candidates.sort();
    candidates.reverse();

    candidates
        .into_iter()
        .find_map(|dir| find_amdsmi_in_dir(&dir))
        .ok_or(AmdSmiError::LibraryNotFound)
}
