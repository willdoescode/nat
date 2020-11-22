use std::os::unix::fs::MetadataExt;
use users;

pub fn user(path: std::path::PathBuf) -> String {
  String::from(
    users::get_user_by_uid(path.symlink_metadata().unwrap().uid())
      .unwrap()
      .name()
      .to_string_lossy(),
  )
}
