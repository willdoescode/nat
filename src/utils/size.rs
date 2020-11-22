use humansize::{file_size_opts as options, FileSize};
use std::os::unix::fs::MetadataExt;

pub fn size(file: std::path::PathBuf) -> String {
  std::fs::symlink_metadata(file)
    .unwrap()
    .size()
    .file_size(options::CONVENTIONAL)
    .unwrap()
}
