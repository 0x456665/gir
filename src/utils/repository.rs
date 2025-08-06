use super::file_path::build_file;
use configparser::ini::Ini;
use std::{env, path::PathBuf};

pub struct GitRepository {
    worktree: PathBuf,
    gitdir: PathBuf,
    pub conf: Ini,
}

impl GitRepository {
    pub fn new(path: PathBuf, force: bool) -> Result<GitRepository, String> {
        let worktree = path;
        let gitdir = worktree.join(".git");

        if !(force || gitdir.exists()) {
            return Err(String::from("Not a git repository"));
        }

        let conf = Ini::new();
        let mut repo = GitRepository {
            worktree,
            gitdir,
            conf,
        };

        let cf = build_file(&repo, &["config"], false);
        if let Some(config_path) = cf {
            if config_path.exists() {
                repo.conf.load(config_path).unwrap();
            } else if !force {
                return Err(String::from("Not a git repository"));
            }
        } else if !force {
            return Err(String::from("Not a git repository"));
        }

        if !force {
            if let Some(vers) = repo.conf.getint("core", "repositoryformatversion").unwrap() {
                if vers != 0 {
                    return Err(format!("Unsupported repositoryformatversion: {}", vers));
                }
            }
        }

        Ok(repo)
    }

    pub fn get_worktree(&self) -> PathBuf {
        self.worktree.clone()
    }

    pub fn get_gitdir(&self) -> PathBuf {
        self.gitdir.clone()
    }
}

pub fn repo_default_config() -> Ini {
    let mut conf = Ini::new();
    conf.set("core", "repositoryformatversion", Some("0".to_string()));
    conf.set("core", "filemode", Some("false".to_string()));
    conf.set("core", "bare", Some("false".to_string()));
    conf
}

pub fn repo_find(
    path: Option<&PathBuf>,
    required: Option<bool>,
) -> Result<Option<GitRepository>, String> {
    let required_bool = required.unwrap_or(true);

    let mut path_buf = match path {
        Some(x) => x
            .clone()
            .canonicalize()
            .map_err(|e| format!("Invalid path: {}", e))?,
        None => {
            env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?
        }
    };

    loop {
        if path_buf.parent().is_none() {
            if required_bool {
                return Err(String::from("No Repository found"));
            } else {
                return Ok(None);
            }
        }
        if path_buf.join(".git").exists() {
            return Ok(Some(GitRepository::new(path_buf, false)?));
        } else {
            path_buf.pop();
        }
    }
}
