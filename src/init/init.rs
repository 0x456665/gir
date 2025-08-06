//import utility functions
use crate::utils::{
    file_path::{build_dir, build_file},
    repository::{GitRepository, repo_default_config},
};
use std::{fs, path::PathBuf};

pub fn init_repo(path: PathBuf) -> Result<GitRepository, String> {
    let repo = GitRepository::new(path, true).unwrap();
    let worktree = repo.get_worktree();
    let gitdir = repo.get_gitdir();

    // check if worktree is a existing directory
    if worktree.exists() {
        if !worktree.is_dir() {
            return Err(String::from("Not a directory"));
        }
        // check if worktree has .git and if it contains folders
        if gitdir.exists() && fs::read_dir(gitdir).unwrap().count() > 0 {
            return Err(String::from("Directory not empty"));
        }
    } else {
        fs::create_dir_all(&worktree).unwrap();
    }

    // Fixed: Proper error handling for build_dir calls
    build_dir(&repo, &["branches"], true).map_err(|e| e)?;
    build_dir(&repo, &["objects"], true).map_err(|e| e)?;
    build_dir(&repo, &["refs", "tags"], true).map_err(|e| e)?;
    build_dir(&repo, &["refs", "heads"], true).map_err(|e| e)?;

    fs::write(
        build_file(&repo, &["description"], false)
            .ok_or("Failed to build description file path")?,
        "Unnamed repository; edit this file 'description' to name the repository.\n",
    )
    .unwrap();

    fs::write(
        build_file(&repo, &["HEAD"], false).ok_or("Failed to build HEAD file path")?,
        "ref: refs/heads/master\n",
    )
    .unwrap();

    let path = build_file(&repo, &["config"], false).ok_or("Failed to build config file path")?;
    repo_default_config().write(&path).unwrap();

    Ok(repo)
}
