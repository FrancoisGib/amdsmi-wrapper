extern crate bindgen;
use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::{Path, PathBuf};

use bindgen::Builder;

include!("allowlist.rs");

fn get_rocm_dir() -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(Path::new("/opt")) {
        let mut rocm_dirs: Vec<PathBuf> = entries
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.is_dir()
                    && let Some(entry_name) = path.file_name()
                    && entry_name.to_string_lossy().starts_with("rocm")
                {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        rocm_dirs.sort();
        if let Some(latest_rocm_dir) = rocm_dirs.last() {
            return Some(latest_rocm_dir.to_path_buf());
        }
    }
    None
}

fn get_amdsmi_header_file(rocm_dir: Option<&PathBuf>) -> Result<String> {
    let amdsmi_header_path = "amdsmi.h";

    if let Ok(current_dir) = env::current_dir() {
        let header_path = current_dir.join(amdsmi_header_path);
        if header_path.exists() {
            return Ok(header_path.to_string_lossy().into_owned());
        }
    }

    let parent_path = Path::new("../").join(amdsmi_header_path);
    if parent_path.exists() {
        return Ok(parent_path.to_string_lossy().into_owned());
    }

    if let Some(rocm_dir) = rocm_dir {
        let header_path = rocm_dir.join("include/amd_smi").join(amdsmi_header_path);
        if header_path.exists() {
            return Ok(header_path.to_string_lossy().into_owned());
        }
    }

    Err(Error::new(
        ErrorKind::NotFound,
        "amdsmi.h header file not found",
    ))
}

fn generate_amdsmi_bindings(amdsmi_header_file: &str) {
    let bindings = bindgen::Builder::default()
        .header(amdsmi_header_file)
        .generate_comments(false)
        .prepend_enum_name(false)
        .rustified_enum("^(.*)$")
        .dynamic_library_name("AmdSmiLib")
        .dynamic_link_require_all(false)
        .derive_debug(true)
        .apply_bindings()
        .generate()
        .unwrap();

    let bindings_path = PathBuf::from("src/bindings.rs");
    bindings.write_to_file(&bindings_path).unwrap();
}

trait Bindings: Sized {
    fn apply_bindings(self) -> Builder {
        let mut builder = self.builder();
        for item in ALLOWLIST_STRUCTS {
            builder = builder.allowlist_type(item)
        }

        for item in ALLOWLIST_FUNCTIONS {
            builder = builder.allowlist_function(item)
        }

        for item in ALLOWLIST_CONSTANTS {
            builder = builder.allowlist_item(item)
        }

        builder
    }

    fn builder(self) -> Builder;
}

impl Bindings for Builder {
    fn builder(self) -> Builder {
        self
    }
}

fn main() {
    if env::var("AMDSMI_GENERATE_BINDINGS").is_ok() {
        let rocm_dir = get_rocm_dir();
        let amdsmi_header_file = get_amdsmi_header_file(rocm_dir.as_ref()).unwrap();
        generate_amdsmi_bindings(&amdsmi_header_file);
    }
}
