extern crate pretty_bytes;
extern crate libc;

use ansi_term::Style;
use chrono::{DateTime, Utc};
use pretty_bytes::converter::convert;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::{fs, io};
use structopt::StructOpt;
use termion::color;
use users::{get_current_uid, get_group_by_gid, get_user_by_uid, uid_t, get_current_gid};
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};

mod single;

#[derive(StructOpt, Debug)]
#[structopt(name = "nat", about = "the ls replacement you never knew you needed")]
struct Cli {
  #[structopt(parse(from_os_str), default_value = ".", help = "Give me a directory")]
  path: std::path::PathBuf,

  #[structopt(
    default_value = "",
    short = "f",
    long = "file",
    help = "File to search for"
  )]
  file: String,

  #[structopt(short = "l", long = "headline", help = "enable the headline")]
  headline_on: bool,


  #[structopt(short = "a", help = "hides hidden files")]
  hidden_files: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::from_args();
  let directory = &args.path;
  let headline_on = &args.headline_on;
  let hidden_files = &args.hidden_files;
  let entries = fs::read_dir(directory)?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;

  let mut size_count = 4;
  let mut group_size = 8;
  for s in &entries {
    if convert(fs::symlink_metadata(&s)?.size() as f64).len() > size_count {
      size_count = convert(fs::symlink_metadata(&s)?.size() as f64).len();
    };

    let metadata_uid = fs::symlink_metadata(&s)?.uid();
    let user_name_len = get_user_name(metadata_uid).len();
    if user_name_len > group_size {
      group_size = user_name_len;
    }

  }

  let mut found = false;
 
  if *headline_on {
  draw_headline("permissions", 0, false);
  draw_headline("size", 0, true);
  draw_headline("last modified", 0, true);
  let mut groups_size: i32 = 0;
  if get_group_by_gid(get_current_gid()).unwrap().name().to_str().unwrap().len() - 5 < 1 {
    groups_size = 0;
  } else {
    groups_size = get_group_by_gid(get_current_gid()).unwrap().name().to_str().unwrap().len() as i32 - 5;
  }
  draw_headline("group", groups_size as usize, true);
  let mut user_size: i32 = 0;
  if get_user_by_uid(get_current_uid()).unwrap().name().to_str().unwrap().len() - 4 < 1 {
    user_size = 0;
  } else {
    user_size = get_user_by_uid(get_current_uid()).unwrap().name().to_str().unwrap().len() as i32 - 4;
  }
  draw_headline("user", 0, true);
  draw_headline("name", 0, true);

    print!("\n");
  }

  if &args.file != "" {
    for e in &entries {
      if e
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase()
        .contains(&args.file.to_lowercase())
      {
        let _ = single::single(e, size_count);
        found = true;
      }
    }
    if !found {
      print!("{}", color::Fg(color::Red));
      println!(
        "{}",
        Style::new()
          .bold()
          .paint(format!("{} could not be found", &args.file))
      );
    }
    std::process::exit(1)
  }

  for e in &entries {
    if !&e.file_name().unwrap().to_str().unwrap().starts_with(".") ||*hidden_files {
    let meta = fs::symlink_metadata(&e)?;
    let mode = meta.permissions().mode();
    let mode_count = perms(mode as u16).len();
    
    print!("{}", color::Fg(color::White));

    print!("{}", perms(mode as u16));

    for _ in 0..(size_count - convert(fs::symlink_metadata(&e)?.size() as f64).len()) {
      print!(" ")
    }
    print!("{}", color::Fg(color::Green));
    print!(
      " {}",
      Style::new()
        .bold()
        .paint(convert(fs::symlink_metadata(&e)?.size() as f64))
    );

    if let Ok(time) = e.symlink_metadata()?.modified() {
      print!("{}", color::Fg(color::LightRed));
      let datetime: DateTime<Utc> = time.into();
      print!(" {} ", datetime.format("%d-%m-%Y"));
      print!("{}", datetime.format("%T"))
    }

    print!("{}", color::Fg(color::LightBlue));

    print!(
      " {} ",
      Style::new().bold().paint(
        get_group_by_gid(fs::symlink_metadata(e)?.gid())
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
        get_user_by_uid(fs::symlink_metadata(e)?.uid())
          .unwrap()
          .name()
          .to_str()
          .unwrap()
      )
    );

    print!("{}", color::Fg(color::White));
    if e.symlink_metadata()?.is_dir() {
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
    print!("{}", color::Fg(color::Reset));
    }
  }
  Ok(())
}

fn draw_headline(input: &str, line_length: usize, print_separator: bool) {
    if print_separator {
        print!(" {}", Style::new().underline().paint(input));
    } else {
        print!("{}", Style::new().underline().paint(input));
    }
    draw_line(line_length);
}

fn draw_line(to: usize) {
    for _ in 0..to {
        print!("{}", Style::new().underline().paint(" "));
    }
}

fn get_user_name(uid: uid_t) -> String {
    get_user_by_uid(uid)
        .unwrap()
        .name()
        .to_str()
        .unwrap()
        .to_string()
}


pub fn perms(mode: u16) -> String {
	let user = triplet(mode, S_IRUSR as u16, S_IWUSR as u16, S_IXUSR as u16);
	let group = triplet(mode, S_IRGRP as u16, S_IWGRP as u16, S_IXGRP as u16);
	let other = triplet(mode, S_IROTH as u16, S_IWOTH as u16, S_IXOTH as u16);
	[user, group, other].join("")
}

pub fn triplet(mode: u16, read: u16, write: u16, execute: u16) -> String {
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
