use std::path::Path;

use egui_notify::ToastLevel;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::log::{ErrorLogEntry, ERROR_LOG};

pub fn load_file<
    T: DeserializeOwned,
    F: for<'a> FnOnce(&'a Path) -> std::io::Result<Result<T, E>> + 'static,
    E: serde::de::Error + Into<eyre::Report>,
>(
    file: &Path,
    deserializer: F,
    error: Option<&'static str>,
) -> eyre::Result<T> {
    match deserializer(file) {
        Ok(Ok(o)) => Ok(o),
        Ok(Err(e)) => {
            if let Some(thing) = error {
                let mut error_log = ERROR_LOG.write().unwrap();
                error_log.pending_errors.push(ErrorLogEntry::new(
                    &format!(
                        "Could not parse {thing} file {}:\n{e}",
                        file.to_string_lossy()
                    ),
                    ToastLevel::Warning,
                ));
            }
            Err(e.into())
        }
        Err(e) => {
            if let Some(thing) = error {
                let mut error_log = ERROR_LOG.write().unwrap();
                error_log.pending_errors.push(ErrorLogEntry::new(
                    &format!(
                        "Could not load {thing} file {}:\n{e}",
                        file.to_string_lossy()
                    ),
                    ToastLevel::Warning,
                ));
            }
            Err(e.into())
        }
    }
}

pub fn load_toml<T: DeserializeOwned>(file: &Path, error: Option<&'static str>) -> eyre::Result<T> {
    load_file(
        file,
        |file| std::fs::read_to_string(file).map(|c| toml::from_str(&c)),
        error,
    )
}

pub fn load_msgpack<T: DeserializeOwned>(
    file: &Path,
    error: Option<&'static str>,
) -> eyre::Result<T> {
    load_file(
        file,
        |file| std::fs::read(file).map(|c| rmp_serde::from_slice(&c)),
        error,
    )
}

pub fn save_file<
    T: Serialize,
    F: FnOnce(&T) -> Result<A, E>,
    A: AsRef<[u8]>,
    E: serde::ser::Error + Into<eyre::Report>,
>(
    o: &T,
    serializer: F,
    file: &Path,
    error: Option<&'static str>,
) -> eyre::Result<()> {
    match serializer(o).map(move |s| std::fs::write(file, s)) {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => {
            if let Some(thing) = error {
                let mut error_log = ERROR_LOG.write().unwrap();
                error_log.pending_errors.push(ErrorLogEntry::new(
                    &format!(
                        "Could not write {thing} file {}:\n{e}",
                        file.to_string_lossy()
                    ),
                    ToastLevel::Warning,
                ));
            }
            Err(e.into())
        }
        Err(e) => {
            if let Some(thing) = error {
                let mut error_log = ERROR_LOG.write().unwrap();
                error_log.pending_errors.push(ErrorLogEntry::new(
                    &format!(
                        "Could not serialise {thing} file {}:\n{e}",
                        file.to_string_lossy()
                    ),
                    ToastLevel::Warning,
                ));
            }
            Err(e.into())
        }
    }
}

pub fn save_toml<T: Serialize>(
    o: &T,
    file: &Path,
    error: Option<&'static str>,
) -> eyre::Result<()> {
    save_file(o, |o| toml::to_string_pretty(o), file, error)
}

pub fn save_toml_with_header<T: Serialize>(
    o: &T,
    file: &Path,
    header: &'static str,
    error: Option<&'static str>,
) -> eyre::Result<()> {
    save_file(
        o,
        |o| toml::to_string_pretty(o).map(|a| format!("{header}\n\n{a}")),
        file,
        error,
    )
}

pub fn save_msgpack<T: Serialize>(
    o: &T,
    file: &Path,
    error: Option<&'static str>,
) -> eyre::Result<()> {
    save_file(o, |o| rmp_serde::to_vec_named(o), file, error)
}