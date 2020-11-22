use users;
use std::os::unix::fs::{MetadataExt};

pub fn group(path: std::path::PathBuf) -> String {
  String::from(users::get_group_by_gid(path.symlink_metadata().unwrap().gid()).unwrap().name().to_string_lossy())
}
