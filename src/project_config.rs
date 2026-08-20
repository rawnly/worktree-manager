use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::git;

#[derive(Deserialize)]
pub struct Hooks {
    pub on_create: Option<Vec<String>>,
    pub on_delete: Option<Vec<String>>,
}

impl Default for Hooks {
    fn default() -> Self {
        Self {
            on_create: None,
            on_delete: None,
        }
    }
}

#[derive(Deserialize)]
pub struct ProjectConfig {
    pub hooks: Option<Hooks>,
}

impl ProjectConfig {
    fn default() -> Self {
        Self { hooks: None }
    }

    // config must live in the bare-repo dir
    fn path() -> anyhow::Result<PathBuf> {
        let project_root = git::worktree_root()?;
        let config_path = Path::new(&project_root);

        Ok(config_path.join("bosco_config.yml").to_path_buf())
    }

    pub fn load() -> anyhow::Result<Self> {
        let path = Self::path()?;

        if path.exists() {
            let content = fs::read_to_string(&path)?;
            Ok(serde_yaml::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }
}
