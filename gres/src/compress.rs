use lz4_flex::frame::{FrameDecoder, FrameEncoder};
use soulog::*;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tar::Builder;

const BUFFER_SIZE: usize = 1024 * 1024 * 7; // 7MiB buffer size

pub fn build_tar(path: impl AsRef<Path>, tar_path: impl AsRef<Path>, mut logger: impl Logger) {
    log!((logger) Tar("Building tarball '{}'...", tar_path.as_ref().to_string_lossy()));

    let tar = if_err!((logger) [Tar, err => ("While creating tarball: {err:?}")] {File::create(tar_path)} crash logger.crash());
    let mut builder = Builder::new(tar);
    let path = path.as_ref();

    if !path.is_dir() {
        log!((logger.error) Tar("Directory '{}' doesn't exist", path.to_string_lossy()) as Fatal);
        return logger.crash();
    }

    recursive_tar_append(&mut builder, path, PathBuf::new(), logger.hollow());
    if_err!((logger) [Tar, err => ("While building tar: {err:?}")] {builder.finish()} crash logger.crash());

    log!((logger.vital) Tar("Successfully built tarball!") as Log);
    log!((logger) Tar(""));
}

fn recursive_tar_append(
    builder: &mut Builder<File>,
    path: impl AsRef<Path>,
    tar_path: PathBuf,
    mut logger: impl Logger,
) {
    let read_dir = if_err!((logger) [Tar, err => ("While reading path '{}': {err:?}", path.as_ref().to_string_lossy())] {std::fs::read_dir(&path)} crash logger.crash());

    for entry in read_dir.filter_map(|x| x.ok()) {
        let path = entry.path();
        let file_type = if_err!((logger) [Tar, err => ("While getting file type: {err:?}")] {entry.file_type()} crash logger.crash());
        if file_type.is_file() {
            let entry_name = entry.file_name().to_string_lossy().to_string();

            log!((logger) Tar("Appending file '{entry_name}' to tarball..."));

            let mut file = if_err!((logger) [Tar, err => ("While opening file '{entry_name}': {err:?}")] {File::open(&path)} crash logger.crash());
            if_err!((logger) [Tar, err => ("While appending file '{entry_name}': {err:?}")] {
                builder.append_file(tar_path.join(entry.file_name()), &mut file)
            } crash logger.crash());
        } else if file_type.is_dir() {
            recursive_tar_append(
                builder,
                path,
                tar_path.join(entry.file_name()),
                logger.hollow(),
            );
        }
    }
}

pub fn unpack_tar(path: impl AsRef<Path>, dir_path: impl AsRef<Path>, mut logger: impl Logger) {
    log!((logger) Tar("Unpacking tarball '{}'...", path.as_ref().to_string_lossy()));
    let path = path.as_ref();

    if !path.is_file() {
        log!((logger.error) Tar("File '{}' doesn't exist", path.to_string_lossy()) as Fatal);
        return logger.crash();
    }

    let tar = if_err!((logger) [Tar, err => ("While opening tarball: {err:?}")] {File::open(path)} crash logger.crash());
    let mut archive = tar::Archive::new(tar);
    let dir_path = dir_path.as_ref();
    let _ = fs::create_dir_all(dir_path);

    let entries = if_err!((logger) [Tar, err => ("While loading tarball entries: {err:?}")] {archive.entries()} crash logger.crash());
    for entry in entries.into_iter() {
        let mut entry = if_err!((logger) [Tar, err => ("While loading entry: {err:?}")] {entry} crash logger.crash());
        let entry_name = if_err!((logger) [Tar, err => ("While getting entry path: {err:?}")] {entry.path()} crash logger.crash()).to_string_lossy().to_string();

        log!((logger) Tar("Unpacking entry '{entry_name}'..."));
        if_err!((logger) [Tar, err => ("While unpacking entry '{entry_name}': {err:?}")] {entry.unpack_in(dir_path)} crash logger.crash());
    }

    log!((logger.vital) Tar("Successfully unpacked tarball!") as Log);
    log!((logger) Tar(""));
}

pub fn compress_file(path: impl AsRef<Path>, out_path: impl AsRef<Path>, mut logger: impl Logger) {
    log!((logger) Compress("Compressing file '{}'...", path.as_ref().to_string_lossy()));
    let path = path.as_ref();

    if !path.is_file() {
        log!((logger.error) Compress("File '{}' doesn't exist", path.to_string_lossy()) as Fatal);
        return logger.crash();
    }

    let mut file = if_err!((logger) [Tar, err => ("While loading file to compress '{}': {err:?}", path.to_string_lossy())] {File::open(path)} crash logger.crash());
    let out = if_err!((logger) [Tar, err => ("While creating archive '{}': {err:?}", out_path.as_ref().to_string_lossy())] {File::create(&out_path)} crash logger.crash());

    let mut encoder = FrameEncoder::new(out);
    let mut buffer = [0u8; BUFFER_SIZE];

    // Read data from input and write compressed output
    let mut chunk: u64 = 0;
    loop {
        log!((logger) Compress("Compressing chunk `{chunk}` of file..."));

        let bytes_read = if_err!((logger) [Compress, err => ("While reading chunk `{chunk}` from file: {err:?}")] {file.read(&mut buffer)} crash logger.crash());
        if bytes_read == 0 {
            break;
        };

        if_err!((logger) [Compress, err => ("While writing chunk to compressed archive: {err:?}")] {encoder.write_all(&buffer[..bytes_read])} crash logger.crash());
        chunk += 1;
    }

    if_err!((logger) [Compress, err => ("While compressing file: {err:?}")] {encoder.finish()} crash logger.crash());
    log!((logger.vital) Compress("Successfully compressed file") as Log);
}

pub fn decompress_file(path: impl AsRef<Path>, out_path: impl AsRef<Path>, mut logger: impl Logger) {
    log!((logger) Decompress("Decompressing file '{}'...", path.as_ref().to_string_lossy()));
    let path = path.as_ref();

    if !path.is_file() {
        log!((logger.error) Decompress("File '{}' doesn't exist", path.to_string_lossy()) as Fatal);
        return logger.crash();
    }

    let file = if_err!((logger) [Tar, err => ("While opening compressed archive: {err:?}")] {File::open(path)} crash logger.crash());
    let mut out = if_err!((logger) [Tar, err => ("While creating uncompressed file: {err:?}")] {File::create(out_path)} crash logger.crash());
    let mut decoder = FrameDecoder::new(file);
    let mut buffer = [0u8; BUFFER_SIZE];

    // Read compressed data and write decompressed version
    let mut chunk: u64 = 0;
    loop {
        log!((logger) Decompress("Decompressing chunk `{chunk}` of archive..."));

        let bytes_read = if_err!((logger) [Decompress, err => ("While reading chunk `{chunk}` from archive: {err:?}")] {decoder.read(&mut buffer)} crash logger.crash());
        if bytes_read == 0 {
            break;
        };
        
        if_err!((logger) [Decompress, err => ("While writing chunk to decompressed file: {err:?}")] {out.write_all(&buffer[..bytes_read])} crash logger.crash());
        chunk += 1;
    }

    if_err!((logger) [Decompress, err => ("While compressing file: {err:?}")] {out.flush()} crash logger.crash());
    log!((logger.vital) Decompress("Successfully decompressed archive") as Log);
}
