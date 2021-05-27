use std::os::unix::fs::MetadataExt;

pub fn group(path: std::path::PathBuf) -> String {
  let group = users::get_group_by_gid(path.symlink_metadata().unwrap().gid());
    if  let Some(g) = group {
      String::from(g.name().to_string_lossy())
    } else {
      String::from(" ")
    }
}

pub fn user(path: std::path::PathBuf) -> String {
  let user = users::get_user_by_uid(path.symlink_metadata().unwrap().uid());
  if let Some(u) = user {
    String::from(u.name().to_string_lossy())
  } else {
    String::from(" ")
  }
}
