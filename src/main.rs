extern crate pretty_bytes;

use ansi_term::Style;
use chrono::{DateTime, Utc};
use pretty_bytes::converter::convert;
use std::os::unix::fs::MetadataExt;
use std::{fs, io};
use structopt::StructOpt;
use termion::color;
use users::{get_current_uid, get_group_by_gid, get_user_by_uid, uid_t, get_current_gid};

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::from_args();
  let mut directory = &args.path;

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

  draw_headline("permissions", 2, false);
  draw_headline("size", size_count - 4, true);
  draw_headline("last modified", 6, true);
  draw_headline("group",get_group_by_gid(get_current_gid()).unwrap().name().to_str().unwrap().len() - 5, true);
  draw_headline("user", get_user_name(get_current_uid()).len() - 4, true);
  draw_headline("name", 0, true);

  print!("\n");

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
    let meta = fs::symlink_metadata(&e)?;
    let mode = meta.mode();
    let user_has_write_access = mode & 0o200;
    let user_has_read_write_access = mode & 0o600;
    let group_has_read_access = mode & 0o040;
    let others_have_exec_access = mode & 0o001;
    let mut mode_count = 0;
    if user_has_write_access == 128 {
      print!("{}", color::Fg(color::Red));
      print!("w");
      print!("{}", color::Fg(color::White));
      print!("-");
      mode_count += 2;
    }
    if user_has_read_write_access == 384 {
      print!("{}", color::Fg(color::LightYellow));
      print!("r");
      print!("{}", color::Fg(color::LightRed));
      print!("w");
      print!("{}", color::Fg(color::White));
      print!("-");
      mode_count += 3;
    }
    if group_has_read_access == 32 {
      print!("{}", color::Fg(color::Green));
      print!("x");
      print!("{}", color::Fg(color::LightYellow));
      print!("a");
      print!("{}", color::Fg(color::White));
      print!("-");
      mode_count += 3;
    }
    if others_have_exec_access == 1 {
      print!("{}", color::Fg(color::Yellow));
      print!("xw");
      print!("{}", color::Fg(color::White));
      print!("-");
      mode_count += 3;
    }
    print!("{}", color::Fg(color::White));
    print!("-@");
    mode_count += 2;
    for _ in 0..(13 - mode_count) {
      print!(" ")
    }

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
