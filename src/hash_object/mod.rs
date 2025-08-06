// def cmd_hash_object(args):
//     if args.write:
//         repo = repo_find()
//     else:
//         repo = None

//     with open(args.path, "rb") as fd:
//         sha = object_hash(fd, args.type.encode(), repo)
//         print(sha)
//
use crate::utils::{git_object, repository};
use std::fs;

pub fn hash_object(path: &str, object_type: Option<&str>, write: bool) {
    let repo = if let true = write {
        repository::repo_find(None, None).unwrap()
    } else {
        None
    };

    let dir = fs::read(path).expect("failed to read object");
    let sha = git_object::object_hash(dir, object_type.unwrap(), repo.as_ref());
    println!("{:?}", sha)
}
