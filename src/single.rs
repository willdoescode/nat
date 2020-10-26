use crate::file_perms;
use crate::file_size;
use crate::time_mod;
use crate::show_group_name;
use crate::show_user_name;
use crate::show_file_name;


pub fn single(e: &std::path::PathBuf, size_count: usize, wide_mode: bool) -> Result<(), Box<dyn std::error::Error>> {
  let _ = file_perms(&e);
  let _ = file_size(size_count, &e);
  let _ = time_mod(e);
  let _ = show_group_name(e);
  let _ = show_user_name(e);
  let _ = show_file_name(&e, wide_mode);
  Ok(())
}
