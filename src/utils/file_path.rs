use super::repository::*;
use std::{fs, path::PathBuf};

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
