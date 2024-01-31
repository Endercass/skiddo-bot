use std::io::Write;

use deno_core::error::AnyError;
use deno_core::extension;
use deno_core::op2;
use deno_core::OpState;
use gotham::state::{State, StateData};
use scorched::{logf, LogData, LogImportance};
use serde::{Deserialize, Serialize};
use vfs::impls::memory::MemoryFS;
use vfs::FileSystem;
use vfs::VfsMetadata;

#[derive(StateData)]
pub struct SkiddoState {
    pub fs: MemoryFS,
}

#[derive(Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[op2]
fn op_log(#[serde] level: LogLevel, #[string] msg: String) {
    match level {
        LogLevel::Debug => logf!(Debug, "JS: {}", msg),
        LogLevel::Info => logf!(Info, "JS: {}", msg),
        LogLevel::Warning => logf!(Warning, "JS: {}", msg),
        LogLevel::Error => logf!(Error, "JS: {}", msg),
    };
}

#[op2(fast)]
fn op_init_state(state: &mut OpState) {
    let skiddo_state = SkiddoState {
        fs: MemoryFS::new(),
    };
    // Iniiialize the filesystem with test file
    let mut file = skiddo_state.fs.create_file("/test.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();

    state.put(skiddo_state);
}

#[op2(fast)]
fn op_fs_create_file(state: &mut OpState, #[string] path: String) -> Result<(), AnyError> {
    let skiddo_state = state.borrow::<SkiddoState>();
    skiddo_state.fs.create_file(&path)?;
    Ok(())
}

#[op2]
#[buffer]
fn op_fs_read_file(state: &mut OpState, #[string] path: String) -> Result<Vec<u8>, AnyError> {
    let skiddo_state = state.borrow::<SkiddoState>();
    let mut file = skiddo_state.fs.open_file(&path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

#[op2(fast)]
fn op_fs_write_file(
    state: &mut OpState,
    #[string] path: String,
    #[buffer] buf: &[u8],
) -> Result<(), AnyError> {
    let skiddo_state = state.borrow::<SkiddoState>();
    if skiddo_state.fs.exists(&path)? {
        skiddo_state.fs.remove_file(&path)?;
    }
    let mut file = skiddo_state.fs.create_file(&path)?;
    file.write_all(buf)?;
    Ok(())
}

#[op2(fast)]
fn op_fs_remove_file(state: &mut OpState, #[string] path: String) -> Result<(), AnyError> {
    let skiddo_state = state.borrow::<SkiddoState>();
    skiddo_state.fs.remove_file(&path)?;
    Ok(())
}

#[op2(fast)]
fn op_fs_create_dir(state: &mut OpState, #[string] path: String) -> Result<(), AnyError> {
    let skiddo_state = state.borrow::<SkiddoState>();
    skiddo_state.fs.create_dir(&path)?;
    Ok(())
}

#[op2(fast)]
fn op_fs_remove_dir(state: &mut OpState, #[string] path: String) -> Result<(), AnyError> {
    let skiddo_state = state.borrow::<SkiddoState>();
    skiddo_state.fs.remove_dir(&path)?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct SkiddoMetadata {
    is_dir: bool,
    len: u64,
}

impl From<VfsMetadata> for SkiddoMetadata {
    fn from(meta: VfsMetadata) -> Self {
        Self {
            is_dir: match meta.file_type {
                vfs::path::VfsFileType::File => false,
                vfs::path::VfsFileType::Directory => true,
            },
            len: meta.len,
        }
    }
}

#[op2]
#[serde]
fn op_fs_stat(state: &mut OpState, #[string] path: String) -> Result<SkiddoMetadata, AnyError> {
    let skiddo_state = state.borrow::<SkiddoState>();
    Ok(skiddo_state.fs.metadata(&path)?.into())
}

#[op2(fast)]
fn op_fs_copy(
    state: &mut OpState,
    #[string] from: String,
    #[string] to: String,
) -> Result<(), AnyError> {
    let skiddo_state = state.borrow::<SkiddoState>();
    match skiddo_state.fs.metadata(&from)?.file_type {
        vfs::path::VfsFileType::File => {
            let mut from_file = skiddo_state.fs.open_file(&from)?;
            let mut to_file = skiddo_state.fs.create_file(&to)?;
            std::io::copy(&mut from_file, &mut to_file)?;
        }
        vfs::path::VfsFileType::Directory => {
            // :( TODO
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Copying directories is not supported yet",
            ))?
        }
    }
    Ok(())
}

#[op2(fast)]
fn op_fs_rename(
    state: &mut OpState,
    #[string] from: String,
    #[string] to: String,
) -> Result<(), AnyError> {
    let skiddo_state = state.borrow::<SkiddoState>();
    match skiddo_state.fs.metadata(&from)?.file_type {
        vfs::path::VfsFileType::File => {
            let mut from_file = skiddo_state.fs.open_file(&from)?;
            let mut to_file = skiddo_state.fs.create_file(&to)?;
            std::io::copy(&mut from_file, &mut to_file)?;
            skiddo_state.fs.remove_file(&from)?;
        }
        vfs::path::VfsFileType::Directory => {
            // :( TODO
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Renaming directories is not supported yet",
            ))?
        }
    }
    Ok(())
}

#[op2]
#[string]
fn op_fs_read_dir(state: &mut OpState, #[string] path: String) -> Result<String, AnyError> {
    let skiddo_state = state.borrow::<SkiddoState>();
    let entries: Vec<String> = skiddo_state.fs.read_dir(&path)?.collect();
    Ok(entries.join("\n"))
}

#[op2]
#[string]
fn op_get_skiddo_file() -> Option<String> {
    std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .iter()
        .filter(|arg| arg.starts_with("--skiddo-file="))
        .map(|arg| arg.trim_start_matches("--skiddo-file=").to_string())
        .filter(|s| !s.is_empty())
        .last()
}

extension!(
    skiddo_internal,
    ops = [op_log, op_get_skiddo_file, op_init_state, op_fs_create_file, op_fs_read_file, op_fs_write_file, op_fs_remove_file, op_fs_create_dir, op_fs_read_dir, op_fs_remove_dir, op_fs_stat, op_fs_copy, op_fs_rename],
    esm_entry_point = "ext:skiddo_internal/ext.js",
    esm = [
        dir "src/js_runtime/skiddo_instance/js",
        "ext.js"
    ],
);
