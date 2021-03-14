#![allow(dead_code)]

mod input;
mod text_effects;
mod utils;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use structopt::StructOpt;
use std::cmp::Ordering;

struct Directory {
  paths: Vec<File>,
  args: input::Cli,
}

#[derive(Clone)]
struct File {
  path:      std::path::PathBuf,
  file_type: Vec<PathType>,
  group:     String,
  user:      String,
  modified:  String,
  created:   String,
  size:      String,
  perms:     String,
}

enum DirSortType {
  Name,
  Created,
  Modified,
  Size,
  Not,
}

#[derive(Copy, Clone, Debug)]
enum PathType {
  Dir,
  Symlink,
  Path,
  Pipe,
  CharD,
  BlockD,
  Socket,
}

impl PathType {
  fn new(file: &std::path::PathBuf) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
    let mut return_val = Vec::new();
    if file.symlink_metadata()?.is_dir() {return_val.push(Self::Dir) }
    if file.symlink_metadata()?.file_type().is_symlink() {return_val.push(Self::Symlink)}
    if file.symlink_metadata()?.file_type().is_fifo() {return_val.push(Self::Pipe)}
    if file.symlink_metadata()?.file_type().is_char_device() {return_val.push(Self::CharD)}
    if file.symlink_metadata()?.file_type().is_block_device() {return_val.push(Self::BlockD)}
    if file.symlink_metadata()?.file_type().is_socket() {return_val.push(Self::Socket)}
    if return_val.is_empty() {return_val.push(Self::Path)}

    Ok(return_val)
  }

  fn create_letter(&self, letter: &str) -> String {
    format!(
      "{}{}{}{}",
      self.get_color_for_type(),
      letter,
      termion::color::Fg(termion::color::Reset),
      termion::color::Bg(termion::color::Reset)
    )
  }

  fn get_letter_for_type(&self) -> String {
    match self {
      Self::Dir     => self.create_letter("d"),
      Self::Symlink => self.create_letter("l"),
      Self::Pipe    => self.create_letter("|"),
      Self::CharD   => self.create_letter("c"),
      Self::BlockD  => self.create_letter("b"),
      Self::Socket  => self.create_letter("s"),
      _             => self.create_letter("."),
    }
  }

  fn get_color_for_type(&self) -> String {
    match self {
      Self::Dir     => format!("{}", termion::color::Fg(termion::color::LightBlue)),
      Self::Symlink => format!("{}", termion::color::Fg(termion::color::LightMagenta)),
      Self::Path    => format!("{}", termion::color::Fg(termion::color::White)),
      Self::Pipe    => format!("{}", termion::color::Fg(termion::color::Yellow)),
      Self::CharD   => format!("{}{}", termion::color::Bg(termion::color::Yellow), termion::color::Fg(termion::color::LightBlue)),
      Self::BlockD  => format!("{}", termion::color::Fg(termion::color::LightGreen)),
      Self::Socket  => format!("{}", termion::color::Fg(termion::color::LightRed)),
    }
  }

  fn get_text_traits_for_type(&self, name: &str, file: &std::path::PathBuf) -> String {
    match self {
      Self::Dir     => text_effects::bold(&format!( "{}{}/", name, termion::color::Fg(termion::color::White))),
      Self::Symlink => text_effects::italic(&format!( "{} -> {}", name, std::fs::read_link(file).unwrap().display().to_string())),
      Self::Path    => text_effects::bold(name),
      Self::Pipe    => text_effects::bold(&format!( "{}{}", name, termion::color::Fg(termion::color::White))),
      Self::CharD   => text_effects::bold(name),
      Self::BlockD  => text_effects::bold(name),
      Self::Socket  => text_effects::bold(&format!( "{}{}", name, termion::color::Fg(termion::color::White))),
    }
  }
}

impl File {
  fn new(file: std::path::PathBuf, time_format: String) -> Self {
    Self {
      group:     utils::group(file.to_path_buf()),
      user:      utils::user(file.to_path_buf()),
      modified:  utils::file_times::modified(file.to_path_buf(), time_format.to_owned()),
      created:   utils::file_times::created(file.to_path_buf(), time_format),
      size:      utils::size::size(file.to_path_buf()),
      perms:     utils::perms::perms(file.to_path_buf()),
      file_type: PathType::new(&file).unwrap(),
      path: file,
    }
  }
}

fn get_sort_type(sort_t: [bool; 4]) -> DirSortType {
  for (i, t) in sort_t.iter().enumerate() {
    if *t {
      match i {
        0 => return DirSortType::Name,
        1 => return DirSortType::Created,
        2 => return DirSortType::Modified,
        3 => return DirSortType::Size,
        _ => (),
      }
    }
  }
  DirSortType::Not
}

impl Directory {
  fn new(args: input::Cli) -> Result<Self, Box<dyn std::error::Error>> {
    let dir = &args.dir;

    if !std::path::Path::new(&dir).exists() {
	    return Err(
        Box::new(
	        std::io::Error::from_raw_os_error(2)
        )
      )
    }

    if !std::path::Path::new(&dir).is_dir() {
      let f = File::new(dir.to_owned(), args.time_format);
      match args.long {
        true => print!("{:?}", f),
        _ => print!("{}", f)
      }
      std::process::exit(0)
    }

    let paths = std::fs::read_dir(dir)?
        .map(|res| res.map(|e| File::new(
          e.path(), args.time_format.to_owned()
            )
          )
        )
        .collect::<Result<Vec<File>, std::io::Error>>()?;
      Ok(Self { paths, args })
  }


  fn sort_directory_then_path(&mut self) {
    let new = &self.paths;
    let mut newer = Vec::new();
    let mut directories = Vec::new();
    for (i, f) in new.iter().enumerate() {
      if f.path.symlink_metadata().unwrap().is_dir() {
        directories.push(new[i].to_owned());
      } else {
        newer.push(new[i].to_owned())
      }
    }

    match get_sort_type([
      self.args.name,
      self.args.created,
      self.args.modified,
      self.args.size,
    ]) {
      DirSortType::Name => {
        name_sort(&mut directories);
        name_sort(&mut newer)
      }
      DirSortType::Created => {
        create_sort(&mut directories);
        create_sort(&mut newer)
      }
      DirSortType::Modified => {
        modified_sort(&mut directories);
        modified_sort(&mut newer)
      }
      DirSortType::Size => {
        size_sort(&mut directories);
        size_sort(&mut newer)
      }
      DirSortType::Not => (),
    }

    directories.append(&mut newer);
    self.paths = directories;
  }

  fn sort_paths(&mut self) {
    match get_sort_type([
      self.args.name,
      self.args.created,
      self.args.modified,
      self.args.size,
    ]) {
      DirSortType::Name     => sort_as(&mut self.paths, |a, b| {
        a.path
          .file_name()
          .unwrap()
          .to_str()
          .unwrap()
          .to_lowercase()
          .cmp(&b.path
            .file_name()
             .unwrap()
            .to_str()
            .unwrap()
            .to_lowercase()
          )
      }),
      DirSortType::Created  => sort_as(&mut self.paths ,|a, b| {
        a.path
          .symlink_metadata()
          .unwrap()
          .created()
          .unwrap()
          .cmp(&b.path
            .symlink_metadata()
            .unwrap()
            .created()
            .unwrap()
          )
      }),
      DirSortType::Modified => sort_as(&mut self.paths, |a, b| {
        a.path
          .symlink_metadata()
          .unwrap()
          .modified()
          .unwrap()
          .cmp(&b.path
            .symlink_metadata()
            .unwrap()
            .modified()
            .unwrap()
          )
      }),
      DirSortType::Size => sort_as(&mut self.paths, |a, b| {
        a.path
          .symlink_metadata()
          .unwrap()
          .size()
          .cmp(&b.path.
            symlink_metadata()
            .unwrap()
            .size()
          )
      }),
      DirSortType::Not => (),
    }
  }


  fn sort(&mut self) {
    match self.args.gdf {
      true  => self.sort_directory_then_path(),
      false => self.sort_paths(),
    }
  }

  fn add_space(&mut self) {
    let mut gs = 0;
    let mut us = 0;
    let mut ss = 0;
    for p in self.paths.iter() {
      if p.group.len() > gs {
        gs = p.group.len()
      }
      if p.user.len() > us {
        us = p.user.len()
      }
      if p.size.len() > ss {
        ss = p.size.len()
      }
    }

    for p in 0..self.paths.iter().len() {
      let ghold = self.paths[p].group.to_owned();
      let uhold = self.paths[p].user.to_owned();
      let shold = self.paths[p].size.to_owned();
      let mut gwidth = String::new();
      for _ in 0..(gs - ghold.len() + 1) {
        gwidth.push(' ');
      }
      let mut uwidth = String::new();
      for _ in 0..(us - uhold.len() + 1) {
        uwidth.push(' ')
      }
      let mut swidth = String::new();
      for _ in 0..(ss - shold.len() + 1) {
        swidth.push(' ')
      }
      self.paths[p].group = format!("{}{}", ghold, gwidth);
      self.paths[p].user  = format!("{}{}", uhold, uwidth);
      self.paths[p].size  = format!("{}{}", swidth, shold);
    }
  }

  fn setup(&mut self) -> &mut Directory {
    self.sort();
    self.add_space();
	  self
  }
}

fn sort_as<T>(files: &mut Vec<File>, sort_method: T)
  where T: Fn(&File, &File) -> Ordering {
  files.sort_by(sort_method)
}

fn name_sort(dir: &mut Vec<File>) {
  dir.sort_by(|a, b| {
    a.path
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .to_lowercase()
      .cmp(&b.path.file_name().unwrap().to_str().unwrap().to_lowercase())
  })
}

fn create_sort(dir: &mut Vec<File>) {
  dir.sort_by(|a, b| {
    a.path
      .symlink_metadata()
      .unwrap()
      .created()
      .unwrap()
      .cmp(&b.path.symlink_metadata().unwrap().created().unwrap())
  })
}

fn modified_sort(dir: &mut Vec<File>) {
  dir.sort_by(|a, b| {
    a.path
      .symlink_metadata()
      .unwrap()
      .modified()
      .unwrap()
      .cmp(&b.path.symlink_metadata().unwrap().modified().unwrap())
  })
}

fn size_sort(dir: &mut Vec<File>) {
  dir.sort_by(|a, b| {
    a.path
      .symlink_metadata()
      .unwrap()
      .size()
      .cmp(&b.path.symlink_metadata().unwrap().size())
  })
}

impl std::fmt::Display for File {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut res = String::new();
    for (i, v) in self.file_type.iter().enumerate() {
      if i == 0 {
        res = format!(
          "{}{}",
          v.get_color_for_type(),
          v.get_text_traits_for_type(
            &self.path.
              components()
              .next_back()
              .unwrap()
              .as_os_str()
              .to_string_lossy()
              .to_string(),
            &self.path
          )
        );
      } else {
        res = format!(
          "{}{}",
          v.get_color_for_type(),
          v.get_text_traits_for_type(&res, &self.path)
        );
      }
    }
    write!(f, "{}", res)
  }
}

impl std::fmt::Debug for File {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut res = String::new();
    for (i, v) in self.file_type.iter().enumerate() {
      if i == 0 {
        res = v.get_text_traits_for_type(
          &self.path
            .components()
            .next_back()
            .unwrap()
            .as_os_str()
            .to_string_lossy()
            .to_string(),
          &self.path
        );
        res = format!("{}{}", v.get_color_for_type(), res);
      } else {
        res = v.get_text_traits_for_type(&res, &self.path);
        res = format!("{}{}", v.get_color_for_type(), res);
      }
    }

	  let time = if input::Cli::from_args().created_time { &self.created }
    else { &self.modified };

    writeln!(f, "{} {green}{} {yellow}{} {blue} {}{} {}",
      self.perms, self.size, self.user, self.group, time, res,
      green = termion::color::Fg(termion::color::LightGreen),
      yellow = termion::color::Fg(termion::color::Yellow),
      blue = termion::color::Fg(termion::color::Blue),
    )
  }
}

impl std::fmt::Display for Directory {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    Ok(for i in self.paths.iter() {
      match self.args.long {
        true => write!(f, "{:?}", i)?,
        _    => write!(f, "{} ", i)?,
      }
    })
  }
}

fn main() {
  println!("{}",
    match Directory::new(input::Cli::from_args()) {
      Ok(mut res) => format!("{}", res.setup()),
      Err(err) => format!("{}{}", termion::color::Fg(termion::color::Red), err)
    }
  );
}

#[cfg(test)]
mod tests;
