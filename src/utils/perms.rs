use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::os::unix::fs::PermissionsExt;

pub fn perms(file: std::path::PathBuf) -> String {
  let mode = file.symlink_metadata().unwrap().permissions().mode() as u16;
  let user = masking(mode, S_IRUSR as u16, S_IWUSR as u16, S_IXUSR as u16);
  let group = masking(mode, S_IRGRP as u16, S_IWGRP as u16, S_IXGRP as u16);
  let other = masking(mode, S_IROTH as u16, S_IWOTH as u16, S_IXOTH as u16);
  let f = crate::PathType::new(&file).unwrap()[0].get_letter_for_type();
  [f, user, group, other].join("")
}

fn masking(mode: u16, read: u16, write: u16, execute: u16) -> String {
  match (mode & read, mode & write, mode & execute) {
    (0, 0, 0) => format!(
      "{}-{}-{}-{}",
      termion::color::Fg(termion::color::LightBlack),
      termion::color::Fg(termion::color::LightBlack),
      termion::color::Fg(termion::color::LightBlack),
      termion::color::Fg(termion::color::Reset)
    ),
    (_, 0, 0) => format!(
      "{}r{}-{}-{}",
      termion::color::Fg(termion::color::Yellow),
      termion::color::Fg(termion::color::LightBlack),
      termion::color::Fg(termion::color::LightBlack),
      termion::color::Fg(termion::color::Reset)
    ),
    (0, _, 0) => format!(
      "{}-{}w{}-{}",
      termion::color::Fg(termion::color::LightBlack),
      termion::color::Fg(termion::color::LightRed),
      termion::color::Fg(termion::color::LightBlack),
      termion::color::Fg(termion::color::Reset)
    ),
    (0, 0, _) => format!(
      "{}-{}-{}x{}",
      termion::color::Fg(termion::color::LightBlack),
      termion::color::Fg(termion::color::LightBlack),
      termion::color::Fg(termion::color::LightGreen),
      termion::color::Fg(termion::color::Reset)
    ),
    (_, 0, _) => format!(
      "{}r{}-{}x{}",
      termion::color::Fg(termion::color::Yellow),
      termion::color::Fg(termion::color::LightBlack),
      termion::color::Fg(termion::color::LightGreen),
      termion::color::Fg(termion::color::Reset)
    ),
    (_, _, 0) => format!(
      "{}r{}w{}-{}",
      termion::color::Fg(termion::color::Yellow),
      termion::color::Fg(termion::color::LightRed),
      termion::color::Fg(termion::color::LightBlack),
      termion::color::Fg(termion::color::Reset)
    ),
    (0, _, _) => format!(
      "{}-{}w{}x{}",
      termion::color::Fg(termion::color::LightBlack),
      termion::color::Fg(termion::color::LightRed),
      termion::color::Fg(termion::color::LightGreen),
      termion::color::Fg(termion::color::Reset)
    ),
    (_, _, _) => format!(
      "{}r{}w{}x{}",
      termion::color::Fg(termion::color::Yellow),
      termion::color::Fg(termion::color::LightRed),
      termion::color::Fg(termion::color::LightGreen),
      termion::color::Fg(termion::color::Reset)
    ),
  }
  .to_string()
}
