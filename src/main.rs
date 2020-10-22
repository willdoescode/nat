use chrono::{DateTime, Utc};
use std::os::unix::fs::MetadataExt;
use std::{env, fs, io};
use termion::color;
use users::{get_current_uid, get_user_by_uid};

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();
  let mut directory = ".";
  if args.len() > 1 {
    directory = &args[1]
  }
  println!("{}", directory);

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
    if user_has_write_access == 128 {
      print!("{}", color::Fg(color::Red));
      print!("w");
      print!("{}", color::Fg(color::White));
      print!("-");
    }
    if user_has_read_write_access == 384 {
      print!("{}", color::Fg(color::Blue));
      print!("rw");
      print!("{}", color::Fg(color::White));
      print!("-");
    }
    if group_has_read_access == 32 {
      print!("{}", color::Fg(color::Red));
      print!("xa");
      print!("{}", color::Fg(color::White));
      print!("-");
    }
    if others_have_exec_access == 1 {
      print!("{}", color::Fg(color::Yellow));
      print!("xw");
      print!("{}", color::Fg(color::White));
      print!("-");
    }
    print!("{}", color::Fg(color::White));
    print!("-@");
    print!("{}", color::Fg(color::Green));
    if fs::metadata(&e)?.size() > 1000 {
      let mut first = fs::metadata(&e)?.size() / 1000;
      let mut second = fs::metadata(&e)?.size() % 1000;
      if second + 1000 > 500 {
        first += 1;
        second = 0;
      }

      print!(" {}.{}", first, second);
      print!("{}", color::Fg(color::Yellow));
      print!("k");
    } else {
      print!(" {:?}", fs::metadata(&e)?.size());
    }

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
    print!(" {:?} ", get_user_by_uid(get_current_uid()).unwrap().name());

    print!("{}", color::Fg(color::White));
    if e.metadata()?.is_dir() {
      println!("{}/", &e.display().to_string()[2..]);
    } else {
      println!("{}", &e.display().to_string()[2..]);
    }
  }
  Ok(())
}
