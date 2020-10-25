extern crate pretty_bytes;
use ansi_term::Style;
use chrono::{DateTime, Utc};
use pretty_bytes::converter::convert;
use std::fs;
use std::os::unix::fs::MetadataExt;
use termion::color;
use users::{get_user_by_uid, get_group_by_gid};
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};

pub fn single(e: &std::path::PathBuf, size_count: usize) -> Result<(), Box<dyn std::error::Error>> {
  let meta = fs::symlink_metadata(&e)?;
  let mode = meta.mode();
  let mut mode_count = 0;

  let mode_count = perms(mode as u16).len();

  print!("{}", color::Fg(color::White));

  print!("{}", perms(mode as u16));


  for _ in 0..(13 - mode_count) {
    print!(" ")
  }

  for _ in 0..(size_count - convert(fs::metadata(&e)?.size() as f64).len()) {
    print!(" ")
  }
  print!("{}", color::Fg(color::Green));
  print!(
    " {}",
    Style::new()
    .bold()
    .paint(convert(fs::metadata(&e)?.size() as f64))
  );

  if let Ok(time) = e.metadata()?.modified() {
    print!("{}", color::Fg(color::LightRed));
    let datetime: DateTime<Utc> = time.into();
    print!(" {} ", datetime.format("%d-%m-%Y"));
    print!("{}", datetime.format("%T"))
  }

  print!("{}", color::Fg(color::Yellow));

  print!(
    " {} ",
    Style::new().bold().paint(
      get_group_by_gid(fs::metadata(e)?.gid())
      .unwrap()
      .name()
      .to_str()
      .unwrap()
    )
  );

  print!("{}", color::Fg(color::LightYellow));

  print!(
    "{} ",
    Style::new().bold().paint(
      get_user_by_uid(fs::metadata(e)?.uid())
      .unwrap()
      .name()
      .to_str()
      .unwrap()
    )
  );

  print!("{}", color::Fg(color::White));
  if e.metadata()?.is_dir() {
    print!("{}", color::Fg(color::LightBlue));
    println!("{}/", &e.file_name().unwrap().to_str().unwrap());
  } else {
    print!("{}", color::Fg(color::LightGreen));
    println!(
      "{}",
      Style::new()
      .bold()
      .paint(e.file_name().unwrap().to_str().unwrap())
    );
  }
  Ok(())
}

fn triplet(mode: u16, read: u16, write: u16, execute: u16) -> String {
  match (mode & read, mode & write, mode & execute) {
    (0, 0, 0) => "---",
    (_, 0, 0) => "r--",
    (0, _, 0) => "-w-",
    (0, 0, _) => "--x",
    (_, 0, _) => "r-x",
    (_, _, 0) => "rw-",
    (0, _, _) => "-wx",
    (_, _, _) => "rwx",
  }.to_string()
}

fn perms(mode: u16) -> String {
  let user = triplet(mode, S_IRUSR as u16, S_IWUSR as u16, S_IXUSR as u16);
  let group = triplet(mode, S_IRGRP as u16, S_IWGRP as u16, S_IXGRP as u16);
  let other = triplet(mode, S_IROTH as u16, S_IWOTH as u16, S_IXOTH as u16);
  [user, group, other].join("")
}
