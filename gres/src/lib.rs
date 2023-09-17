pub mod cli;
pub mod compress;
pub mod logger;

use std::{path::Path, fs};
use compress::*;
use soulog::*;

pub fn compress(set_path: impl AsRef<Path>, out_path: Option<impl AsRef<Path>>, clean: bool) {
    let mut logger = logger::GresLogger;
    let mut path = set_path.as_ref().to_path_buf();

    let out_path = match out_path {
        None => path.with_extension("gca"),
        Some(x) => x.as_ref().to_path_buf(),
    };

    if !path.exists() {
        log!((logger.error) Gres("File '{}' doesn't exist", path.to_string_lossy()) as Fatal);
        return logger.crash();
    }

    let is_dir = path.is_dir();
    if is_dir {
        let new_path = path.with_extension("tmp.tar");
        build_tar(&path, &new_path, logger.hollow());
        if clean { let _ = fs::remove_dir_all(path); } // Cleanup
        path = new_path;
    }

    compress_file(&path, out_path, logger);

    // Cleanup
    if is_dir { let _ = fs::remove_file(&path); }
    if clean {
        if is_dir { let _ = fs::remove_dir_all(set_path); }
        else { let _ = fs::remove_file(path); }
    }
}

pub fn decompress(path: impl AsRef<Path>, set_out_path: impl AsRef<Path>, tar: bool, clean: bool) {
    let mut logger = logger::GresLogger;
    let path = path.as_ref();

    let out_path = if tar {
        set_out_path.as_ref().with_extension("tmp.tar")
    } else { set_out_path.as_ref().to_path_buf() };

    if !path.is_file() {
        log!((logger.error) Gres("Compressed archive '{}' doesn't exist", path.to_string_lossy()) as Fatal);
        return logger.crash();
    }

    decompress_file(path, &out_path, logger.hollow());

    if tar {
        unpack_tar(&out_path, set_out_path, logger.hollow());
        let _ = fs::remove_file(out_path);
    }

    if clean { let _ = fs::remove_file(path);}
}