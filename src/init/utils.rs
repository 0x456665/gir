use configparser::ini::Ini;
use std::{fs, path::PathBuf};

pub fn run() {
    println!("Hello, world!");
}

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

pub fn build_path(repo: &GitRepository, paths: &[&str]) -> PathBuf {
    let mut git_dir = repo.get_gitdir();
    for path in paths.iter() {
        git_dir.push(path);
    }
    git_dir
}

pub fn build_dir(
    repo: &GitRepository,
    paths: &[&str],
    mkdir: bool,
) -> Result<Option<PathBuf>, String> {
    let path = build_path(repo, paths);

    if path.exists() {
        if path.is_dir() {
            return Ok(Some(path));
        } else {
            return Err(String::from("Not a directory"));
        }
    }
    if mkdir {
        fs::create_dir_all(&path).map_err(|e| format!("Failed to create directory: {}", e))?;
        return Ok(Some(path));
    } else {
        return Ok(None);
    }
}

pub fn build_file(repo: &GitRepository, paths: &[&str], mkdir: bool) -> Option<PathBuf> {
    let built_path = build_dir(repo, &paths[..paths.len() - 1], mkdir);

    match built_path {
        Ok(Some(_)) => Some(build_path(repo, paths)),
        Ok(None) => None,
        Err(_) => None,
    }
}

pub fn repo_default_config() -> Ini {
    let mut conf = Ini::new();
    conf.set("core", "repositoryformatversion", Some("0".to_string()));
    conf.set("core", "filemode", Some("false".to_string()));
    conf.set("core", "bare", Some("false".to_string()));
    conf
}
