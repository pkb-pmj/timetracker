use std::{
    path::{Path, PathBuf},
    sync::RwLock,
};

use serde::{Deserialize, Serialize};
use tauri_plugin_vnidrop_fs::VnidropDirTarget;
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    sync: SyncConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncConfig {
    data_dirs: Vec<VnidropDirTarget>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            sync: Default::default(),
        }
    }
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            data_dirs: Default::default(),
        }
    }
}

pub struct ConfigState {
    file_path: PathBuf,
    config: RwLock<AppConfig>,
}

impl ConfigState {
    pub fn new(dir_path: impl Into<PathBuf>) -> Result<Self, Error> {
        let dir_path = dir_path.into();
        let file_path = dir_path.join("config.toml");

        let config = match read_config(&file_path)? {
            Some(config) => config,
            None => {
                std::fs::create_dir_all(&dir_path).map_err(|e| Error::CreateDir {
                    source: e,
                    path: dir_path,
                })?;

                let config = Default::default();
                write_config(&file_path, &config)?;
                config
            }
        };

        Ok(Self {
            file_path,
            config: RwLock::new(config),
        })
    }

    pub fn get(&self) -> AppConfig {
        self.config.read().unwrap().clone()
    }

    pub fn update(&self, f: impl FnOnce(&mut AppConfig)) -> Result<(), Error> {
        let mut config = self.config.read().unwrap().clone();
        f(&mut config);
        write_config(&self.file_path, &config)
    }
}

fn write_config(path: impl AsRef<Path>, config: &AppConfig) -> Result<(), Error> {
    let serialized = toml::to_string_pretty(config)?;

    std::fs::write(&path, serialized).map_err(|e| Error::Write {
        source: e,
        path: path.as_ref().to_path_buf(),
    })?;

    Ok(())
}

fn read_config(path: impl AsRef<Path>) -> Result<Option<AppConfig>, Error> {
    match std::fs::read_to_string(&path) {
        Ok(s) => Ok(toml::from_str(&s)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(Error::Read {
            source: e,
            path: path.as_ref().to_path_buf(),
        }),
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read config file: {source} {path}")]
    Read {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("failed to create config directory: {source} {path}")]
    CreateDir {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("failed to write config file: {source} {path}")]
    Write {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("failed to serialize config to TOML: {0}")]
    Serialize(#[from] toml::ser::Error),
    #[error("failed to parse TOML config file: {0}")]
    Parse(#[from] toml::de::Error),
}
