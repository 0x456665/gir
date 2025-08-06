use crate::utils::{
    git_object::{GitObjectTrait, object_find, object_read},
    repository::repo_find,
};
use std::io::{self, Write};
// def cmd_cat_file(args):
//     repo = repo_find()
//     cat_file(repo, args.object, fmt=args.type.encode())

// def cat_file(repo, obj, fmt=None):
//     obj = object_read(repo, object_find(repo, obj, fmt=fmt))
//     sys.stdout.buffer.write(obj.serialize())
//

pub fn cat_file(object: &str, fmt: &str) {
    let repo = repo_find(None, None).unwrap().unwrap();
    let find_object = object_find(&repo, object, Some(fmt), Some(true)).unwrap();
    let obj = object_read(&repo, &find_object)
        .unwrap()
        .expect("Object not found");
    io::stdout().write_all(&obj.serialize()).unwrap();
}
