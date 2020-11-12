#![allow(clippy::collapsible_if)]
extern crate libc;
extern crate pretty_bytes;
use ansi_term::Style;
use chrono::prelude::*;
use filetime::FileTime;
use humansize::{file_size_opts as options, FileSize};
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use pretty_bytes::converter::convert;
use std::{
  fs, io,
  os::unix::fs::{MetadataExt, PermissionsExt},
};
use structopt::StructOpt;
use termion::color;
use users::{get_group_by_gid, get_user_by_uid, uid_t};

/// the ls replacement you never knew you needed
#[derive(StructOpt, Debug)]
pub struct Cli {
  /// Give me a directory
  #[structopt(parse(from_os_str), default_value = ".")]
  path: std::path::PathBuf,

  /// Enables helper headline
  #[structopt(short = "l", long = "headline")]
  headline_on: bool,

  /// Shows hidden files
  #[structopt(short = "a", long = "arehidden")]
  hidden_files: bool,

  /// Enables wide mode output
  #[structopt(short, long = "wide")]
  wide_mode: bool,

  /// Disables the file time modified output
  #[structopt(short, long = "time")]
  time_on: bool,

  /// Disables file size output
  #[structopt(short, long = "size")]
  size_on: bool,

  /// Disables the file group output
  #[structopt(short, long = "group")]
  group_on: bool,

  /// Disables the permissions output
  #[structopt(short, long = "perms")]
  perms_on: bool,

  /// Disables the file user output
  #[structopt(short, long = "user")]
  user_on: bool,

  /// Turns off sorting
  #[structopt(short = "n", long = "nsort")]
  is_sorted: bool,

  /// Turns off color output
  #[structopt(short = "c", long = "ncolors")]
  colors_on: bool,

  /// Specify time format https://docs.rs/chrono/*/chrono/format/strftime/index.html
  #[structopt(long = "time-format", default_value = "%b %e %T")]
  time_format: String,

  /// Turns off sorting by name (on by default)
  #[structopt(long="no-name")]
  by_name: bool,

  /// Sorts by files by date modified
  #[structopt(short="m")]
  by_modified: bool,
}

fn output() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::from_args();
  let directory = &args.path;
  let hidden_files = &args.hidden_files;
  let wide_mode = &args.wide_mode;
  let time_on = &args.time_on;
  let size_on = &args.size_on;
  let group_on = &args.group_on;
  let perms_on = &args.perms_on;
  let user_on = &args.user_on;
  let is_sorted = &args.is_sorted;
  let time_format = &args.time_format;
  let colors_on = &args.colors_on;
  let headline_on = &args.headline_on;
  let by_name = &args.by_name;
  let by_modified = &args.by_modified;

  draw_headlines(*headline_on, *perms_on, *size_on, *time_on, *group_on, *user_on);

  let mut singly_found = false;
  if !std::path::Path::new(directory).exists() {
    let mut entries = fs::read_dir(".")?
      .map(|res| res.map(|e| e.path()))
      .collect::<Result<Vec<_>, io::Error>>()?;
    if *by_modified {
      entries.sort_by(|a, b| FileTime::from_last_modification_time(&fs::symlink_metadata(&a).unwrap()).seconds().cmp(&FileTime::from_last_modification_time(&fs::symlink_metadata(&b).unwrap()).seconds())); 
    }
    if !*by_name {
      entries.sort_by(|a, b| a.file_name().unwrap().to_str().unwrap().to_lowercase().cmp(&b.file_name().unwrap().to_str().unwrap().to_lowercase()));
    }

    let mut size_count = 4;
    for s in &entries {
      if convert(fs::symlink_metadata(&s)?.size() as f64).len() > size_count {
        size_count = convert(fs::symlink_metadata(&s)?.size() as f64).len();
      };
    }
    for e in &entries {
      if e
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase()
        .contains(&args.path.display().to_string().to_lowercase())
      {
        let _ = single(e, size_count, *wide_mode, time_format);
        singly_found = true;
      }
    }
    if !singly_found {
      if !*colors_on {
        print!("{}", color::Fg(color::Red));
      }
      println!(
        "{}",
        Style::new().bold().paint(format!(
          "{} could not be found",
          &args.path.display().to_string()
        ))
      );
    }
    std::process::exit(1);
  }

  if !directory.symlink_metadata()?.is_dir() {
    let _ = single(&directory, 4 as usize, *wide_mode, time_format);
    std::process::exit(0);
  }

  let mut entries = fs::read_dir(directory)?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;

    if *by_modified {
      entries.sort_by(|a, b| FileTime::from_last_modification_time(&fs::symlink_metadata(&a).unwrap()).seconds().cmp(&FileTime::from_last_modification_time(&fs::symlink_metadata(&b).unwrap()).seconds())); 
    }
    if !*by_name {
      entries.sort_by(|a, b| a.file_name().unwrap().to_str().unwrap().to_lowercase().cmp(&b.file_name().unwrap().to_str().unwrap().to_lowercase()));
    }
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

  let mut dirs: Vec<&std::path::PathBuf> = vec![];

  for e in &entries {
    if !&e.file_name().unwrap().to_str().unwrap().starts_with('.') || *hidden_files {
      if e.is_file() && !*is_sorted {
        dirs.push(e);
      } else {
        if !perms_on {
          let _ = file_perms(&e);
        }

        if !size_on {
          let _ = file_size(size_count, &e);
        }

        if !time_on {
          let _ = time_mod(e, time_format);
        }

        if !group_on {
          let _ = show_group_name(e);
        }

        if !user_on {
          let _ = show_user_name(e);
        }

        let _ = show_file_name(&e, *wide_mode);
      }
    }
  }

  for e in dirs {
    let _ = single(e, size_count, *wide_mode, time_format);
  }

  Ok(())
}

fn main() {
  let _ = output();
}

pub fn draw_headlines(
  headline_on: bool,
  perms_on: bool,
  size_on: bool,
  time_on: bool,
  group_on: bool,
  user_on: bool,
) {
  if headline_on {
    if !perms_on {
      draw_headline("permissions", 0, false);
    }
    if !size_on {
      draw_headline("size", 0, true);
    }
    if !time_on {
      draw_headline("last modified", 0, true);
    }
    if !group_on {
      draw_headline("group", 0, true);
    }
    if !user_on {
      draw_headline("user", 0, true);
    }
    draw_headline("name", 0, true);
    println!();
  }
}

pub fn file_perms(e: &&std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
  let mode = fs::symlink_metadata(&e)?.permissions().mode();
  if !Cli::from_args().colors_on {
    print!("{}", color::Fg(color::White));
  }
  print!("{} ", perms(mode as u16));
  Ok(())
}

pub fn file_size(
  size_count: usize,
  e: &&std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
  if size_count > 4 {
    for _ in 0..(size_count
      - (fs::symlink_metadata(&e)?.size())
        .file_size(options::CONVENTIONAL)
        .unwrap()
        .len())
    {
      print!(" ");
    }
  }
  if !Cli::from_args().colors_on {
    print!("{}", color::Fg(color::Green));
  }
  print!(
    "{} ",
    Style::new().bold().paint(
      (fs::symlink_metadata(&e)?.size())
        .file_size(options::CONVENTIONAL)
        .unwrap()
    )
  );
  Ok(())
}

pub fn time_mod(
  e: &std::path::PathBuf,
  time_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  if e.symlink_metadata()?.modified().is_ok() {
    let timestamp = fs::symlink_metadata(e)?;
    let naive = NaiveDateTime::from_timestamp(
      FileTime::from_last_modification_time(&timestamp).seconds() as i64,
      0,
    );
    let datetime: DateTime<Local> = DateTime::from_utc(naive, *Local::now().offset());
    if !Cli::from_args().colors_on {
      print!("{}", color::Fg(color::LightRed));
    }
    print!("{} ", datetime.format(time_format));
  }
  Ok(())
}

pub fn show_group_name(e: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
  if !Cli::from_args().colors_on {
    print!("{}", color::Fg(color::LightBlue));
  }

  print!(
    "{} ",
    Style::new().bold().paint(
      match get_group_by_gid(fs::symlink_metadata(e)?.gid()).as_ref() {
        Some(n) => n.name().to_str().unwrap(),
        None => "",
      }
    )
  );
  Ok(())
}

pub fn show_user_name(e: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
  if !Cli::from_args().colors_on {
    print!("{}", color::Fg(color::LightYellow));
  }
  print!(
    "{} ",
    Style::new().bold().paint(
      match get_user_by_uid(fs::symlink_metadata(e)?.uid()).as_ref() {
        Some(n) => n.name().to_str().unwrap(),
        None => "",
      }
    )
  );
  Ok(())
}

pub fn show_file_name(
  e: &&std::path::PathBuf,
  wide_mode: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  let colors_on = Cli::from_args().colors_on;
  if !colors_on {
    print!("{}", color::Fg(color::White));
  }
  if e.symlink_metadata()?.is_dir() {
    if !colors_on {
      print!("{}", color::Fg(color::LightBlue));
    }
    print!("{}/", &e.file_name().unwrap().to_str().unwrap());
    if !wide_mode {
      println!();
    } else {
      print!(" ");
    }
  } else if e.symlink_metadata()?.file_type().is_symlink() {
    if !colors_on {
      print!("{}", color::Fg(color::LightMagenta));
    }
    print!(
      "{} -> ",
      Style::new()
        .bold()
        .paint(e.file_name().unwrap().to_str().unwrap())
    );
    match fs::canonicalize(fs::read_link(e)?) {
      Ok(_n) => {
        if fs::read_link(e)?.is_dir() {
          if !colors_on {
            print!("{}", color::Fg(color::LightBlue));
            print!(
              "{}/",
              fs::canonicalize(fs::read_link(e)?)
                .unwrap()
                .to_str()
                .unwrap()
            )
          }
        } else {
          if !colors_on {
            print!("{}", color::Fg(color::LightGreen));
            print!(
              "{}",
              fs::canonicalize(fs::read_link(e)?)
                .unwrap()
                .to_str()
                .unwrap()
            )
          }
        }
      }
      Err(_err) => {
        if fs::read_link(e)?.is_dir() {
          if !colors_on {
            print!("{}", color::Fg(color::LightBlue));
            print!(
              "{}/",
              fs::read_link(e)?.file_name().unwrap().to_str().unwrap()
            )
          }
        } else {
          if !colors_on {
            print!("{}", color::Fg(color::LightGreen));
            print!(
              "{}",
              fs::read_link(e)?.file_name().unwrap().to_str().unwrap()
            )
          }
        }
      }
    }
    if !wide_mode {
      println!();
    } else {
      print!(" ");
    }
  } else {
    if !colors_on {
      print!("{}", color::Fg(color::LightGreen));
    }
    print!(
      "{}",
      Style::new()
        .bold()
        .paint(e.file_name().unwrap().to_str().unwrap())
    );
    if !wide_mode {
      println!();
    } else {
      print!(" ");
    }
  }
  print!("{}", color::Fg(color::Reset));
  Ok(())
}

pub fn draw_headline(input: &str, line_length: usize, print_separator: bool) {
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

pub fn get_user_name(uid: uid_t) -> String {
  match get_user_by_uid(uid) {
    Some(m) => m.name().to_str().unwrap().to_string(),
    None => "".to_string(),
  }
}

pub fn single(
  e: &std::path::PathBuf,
  size_count: usize,
  wide_mode: bool,
  time_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::from_args();

  if !&args.perms_on {
    let _ = file_perms(&e);
  }

  if !&args.size_on {
    let _ = file_size(size_count, &e);
  }

  if !&args.time_on {
    let _ = time_mod(e, time_format);
  }

  if !&args.group_on {
    let _ = show_group_name(e);
  }

  if !&args.user_on {
    let _ = show_user_name(e);
  }
  let _ = show_file_name(&e, wide_mode);
  Ok(())
}

pub fn perms(mode: u16) -> String {
  let user = triplet(mode, S_IRUSR as u16, S_IWUSR as u16, S_IXUSR as u16);
  let group = triplet(mode, S_IRGRP as u16, S_IWGRP as u16, S_IXGRP as u16);
  let other = triplet(mode, S_IROTH as u16, S_IWOTH as u16, S_IXOTH as u16);
  if !Cli::from_args().colors_on {
    [
      format!("{}{}", color::Fg(color::Blue), user),
      format!("{}{}", color::Fg(color::LightRed), group),
      format!("{}{}", color::Fg(color::Yellow), other),
    ]
    .join("")
  } else {
    [user, group, other].join("")
  }
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
  }
  .to_string()
}
