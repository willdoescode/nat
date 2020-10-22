use chrono::{DateTime, Utc};
use std::os::unix::fs::MetadataExt;
use std::{env, fs, io};
use termion::color;
use users::{get_current_uid, get_user_by_uid};
extern crate pretty_bytes;
use pretty_bytes::converter::convert;

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();
  let mut directory = ".";
  if args.len() > 1 {
    directory = &args[1]
  }

  let entries = fs::read_dir(directory)?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;

  for e in entries {
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
      print!("{}", color::Fg(color::Blue));
      print!("rw");
      print!("{}", color::Fg(color::White));
      print!("-");
      mode_count += 3;
    }
    if group_has_read_access == 32 {
      print!("{}", color::Fg(color::Red));
      print!("xa");
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
    print!("{}", color::Fg(color::Green));
    print!(" {}", convert(fs::metadata(&e)?.size() as f64));

    if let Ok(time) = e.metadata()?.created() {
      print!("{}", color::Fg(color::Blue));
      let datetime: DateTime<Utc> = time.into();
      print!(" {} ", datetime.format("%d-%m-%Y"));
      print!("{}", datetime.format("%T"))
    }
    if let Ok(time) = e.metadata()?.modified() {
      print!("{}", color::Fg(color::Red));
      let datetime: DateTime<Utc> = time.into();
      print!(" {} ", datetime.format("%d-%m-%Y"));
      print!("{}", datetime.format("%T"))
    }

    print!("{}", color::Fg(color::Yellow));
    print!(
      " {} ",
      get_user_by_uid(get_current_uid())
        .unwrap()
        .name()
        .to_str()
        .unwrap()
    );

    print!("{}", color::Fg(color::White));
    if e.metadata()?.is_dir() {
      print!("{}", color::Fg(color::LightBlue));
      println!("{}/", &e.file_name().unwrap().to_str().unwrap());
    } else {
      print!("{}", color::Fg(color::LightGreen));
      println!("{}", &e.file_name().unwrap().to_str().unwrap());
    }
  }
  Ok(())
}
