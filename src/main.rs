extern crate pretty_bytes;
use ansi_term::Style;
use chrono::{DateTime, Utc};
use pretty_bytes::converter::convert;
use std::os::unix::fs::MetadataExt;
use std::{fs, io};
use structopt::StructOpt;
use termion::color;
use users::{get_current_uid, get_group_by_gid, get_user_by_uid};
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
  let directory = &args.path;

  let entries = fs::read_dir(directory)?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;

  let mut size_count = 0;
  let mut group_size = 0;
  for s in &entries {
    if convert(fs::metadata(&s)?.size() as f64).len() > size_count {
      size_count = convert(fs::metadata(&s)?.size() as f64).len();
    };
    if get_user_by_uid(fs::metadata(&s)?.uid())
      .unwrap()
      .name()
      .to_str()
      .unwrap()
      .len()
      > group_size
    {
      group_size = get_user_by_uid(fs::metadata(&s)?.uid())
        .unwrap()
        .name()
        .to_str()
        .unwrap()
        .len()
    }
  }

  let mut found = false;

  print!("{}", Style::new().underline().paint("permissions"));
  for _ in 0..2 {
    print!("{}", Style::new().underline().paint(" "))
  }
  print!(" {}", Style::new().underline().paint("size"));
  for _ in 0..(size_count - 4) {
    print!("{}", Style::new().underline().paint(" "))
  }

  print!(" {}", Style::new().underline().paint("last modified"));

  for _ in 0..6 {
    print!("{}", Style::new().underline().paint(" "))
  }

  print!(" {}", Style::new().underline().paint("group"));
  for _ in 0..(group_size - 8) {
    print!("{}", Style::new().underline().paint(" "));
  }

  print!(" {}", Style::new().underline().paint("user"));

  for _ in 0..(get_user_by_uid(get_current_uid())
    .unwrap()
    .name()
    .to_str()
    .unwrap()
    .len()
    - 4)
  {
    print!("{}", Style::new().underline().paint(" "))
  }

  print!(" {}", Style::new().underline().paint("name"));

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
    let meta = fs::metadata(&e)?;
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
  }
  Ok(())
}
