use super::file_path::build_file;
use super::repository::GitRepository;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::{Read, Write};

/// Read object sha from Git repository repo.
/// Return a GitObject whose exact type depends on the object
pub trait GitObjectTrait {
    fn fmt(&self) -> &'static str;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&self) -> Vec<u8> {
        unimplemented!()
    }
}

pub struct GitCommit(Vec<u8>);
pub struct GitTree(Vec<u8>);
pub struct GitTag(Vec<u8>);
pub struct GitBlob(Vec<u8>);

pub enum GitObject {
    Commit(GitCommit),
    Tree(GitTree),
    Tag(GitTag),
    Blob(GitBlob),
}

impl GitCommit {
    fn new(data: Vec<u8>) -> Self {
        GitCommit(data)
    }
}

impl GitTree {
    fn new(data: Vec<u8>) -> Self {
        GitTree(data)
    }
}

impl GitTag {
    fn new(data: Vec<u8>) -> Self {
        GitTag(data)
    }
}

impl GitBlob {
    fn new(data: Vec<u8>) -> Self {
        GitBlob(data)
    }
}

impl GitObjectTrait for GitCommit {
    fn fmt(&self) -> &'static str {
        "commit"
    }

    fn serialize(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl GitObjectTrait for GitTree {
    fn fmt(&self) -> &'static str {
        "tree"
    }

    fn serialize(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl GitObjectTrait for GitTag {
    fn fmt(&self) -> &'static str {
        "tag"
    }

    fn serialize(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl GitObjectTrait for GitBlob {
    fn fmt(&self) -> &'static str {
        "blob"
    }

    fn serialize(&self) -> Vec<u8> {
        self.0.clone()
    }

    fn deserialize(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl GitObjectTrait for GitObject {
    fn fmt(&self) -> &'static str {
        match self {
            GitObject::Commit(obj) => obj.fmt(),
            GitObject::Tree(obj) => obj.fmt(),
            GitObject::Tag(obj) => obj.fmt(),
            GitObject::Blob(obj) => obj.fmt(),
        }
    }

    fn serialize(&self) -> Vec<u8> {
        match self {
            GitObject::Commit(obj) => obj.serialize(),
            GitObject::Tree(obj) => obj.serialize(),
            GitObject::Tag(obj) => obj.serialize(),
            GitObject::Blob(obj) => obj.serialize(),
        }
    }
}

pub fn object_read(repo: &GitRepository, sha: &str) -> Result<Option<GitObject>, String> {
    // Build path: objects/xx/xxxxxx... (first 2 chars as directory, rest as filename)
    let path = build_file(repo, &["objects", &sha[0..2], &sha[2..]], false)
        .ok_or("Failed to build file path")?;

    if !path.exists() {
        return Ok(None);
    }

    // Read and decompress the file
    let compressed_data = fs::read(&path).map_err(|e| format!("Could not read file: {}", e))?;
    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut raw = Vec::new();
    decoder
        .read_to_end(&mut raw)
        .map_err(|e| format!("Failed to decompress: {}", e))?;

    // Read object type (find first space)
    let x = raw
        .iter()
        .position(|&b| b == b' ')
        .ok_or("Malformed object: no space found")?;
    let fmt = &raw[0..x];

    // Read and validate object size (find null byte after space)
    let y = raw[x..]
        .iter()
        .position(|&b| b == b'\x00')
        .ok_or("Malformed object: no null byte found")?
        + x;

    let size_str =
        std::str::from_utf8(&raw[x + 1..y]).map_err(|_| "Invalid UTF-8 in size field")?;
    let size: usize = size_str.parse().map_err(|_| "Invalid size format")?;

    if size != raw.len() - y - 1 {
        return Err(format!("Malformed object {}: bad length", sha));
    }

    // Pick constructor based on object type
    let content = raw[y + 1..].to_vec();
    let git_object = match fmt {
        b"commit" => GitObject::Commit(GitCommit::new(content)),
        b"tree" => GitObject::Tree(GitTree::new(content)),
        b"tag" => GitObject::Tag(GitTag::new(content)),
        b"blob" => GitObject::Blob(GitBlob::new(content)),
        _ => {
            let fmt_str = std::str::from_utf8(fmt).unwrap_or("<invalid utf8>");
            return Err(format!("Unknown type {} for object {}", fmt_str, sha));
        }
    };

    Ok(Some(git_object))
}

pub fn object_write(
    obj: &impl GitObjectTrait,
    repo: Option<&GitRepository>,
) -> Result<String, String> {
    // Serialize object data
    let data = obj.serialize();

    // Add header: obj.fmt + b' ' + str(len(data)).encode() + b'\x00' + data
    let mut result = Vec::new();
    result.extend_from_slice(obj.fmt().as_bytes()); // obj.fmt
    result.push(b' '); // b' '
    result.extend_from_slice(data.len().to_string().as_bytes()); // str(len(data)).encode()
    result.push(b'\x00'); // b'\x00'
    result.extend_from_slice(&data); // data

    // Compute hash
    let mut hasher = Sha1::new();
    hasher.update(&result);
    let sha = format!("{:x}", hasher.finalize());

    if let Some(repo) = repo {
        // Compute path
        let path = build_file(repo, &["objects", &sha[0..2], &sha[2..]], true)
            .ok_or("Failed to build file path")?;

        if !path.exists() {
            // Compress and write
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder
                .write_all(&result)
                .map_err(|e| format!("Failed to compress: {}", e))?;
            let compressed = encoder
                .finish()
                .map_err(|e| format!("Failed to finish compression: {}", e))?;

            fs::write(&path, compressed).map_err(|e| format!("Failed to write file: {}", e))?;
        }
    }

    Ok(sha)
}

// def object_find(repo, name, fmt=None, follow=True):
//     return name

pub fn object_find(
    repo: &GitRepository,
    name: &str,
    fmt: Option<&str>,
    follow: Option<bool>,
) -> Result<String, String> {
    let _empty = (repo, name, fmt, follow);
    Ok(name.to_string())
}

// def object_hash(fd, fmt, repo=None):
//     """ Hash object, writing it to repo if provided."""
//     data = fd.read()

//     # Choose constructor according to fmt argument
//     match fmt:
//         case b'commit' : obj=GitCommit(data)
//         case b'tree'   : obj=GitTree(data)
//         case b'tag'    : obj=GitTag(data)
//         case b'blob'   : obj=GitBlob(data)
//         case _: raise Exception(f"Unknown type {fmt}!")

//     return object_write(obj, repo)

pub fn object_hash(fd: Vec<u8>, fmt: &str, repo: Option<&GitRepository>) -> Result<String, String> {
    let obj = match fmt {
        "commit" => GitObject::Commit(GitCommit::new(fd)),
        "tree" => GitObject::Tree(GitTree::new(fd)),
        "tag" => GitObject::Tag(GitTag::new(fd)),
        "blob" => GitObject::Blob(GitBlob::new(fd)),
        _ => {
            return Err(format!("Unknown type {}!", fmt));
        }
    };

    object_write(&obj, repo)
}
