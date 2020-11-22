use chrono;
use filetime;

pub fn modified(file: std::path::PathBuf, format: String) -> String {
  let naive = chrono::NaiveDateTime::from_timestamp(
    filetime::FileTime::from_last_modification_time(&file.symlink_metadata().unwrap()).seconds()
      as i64,
    0,
  );

  let datetime: chrono::DateTime<chrono::Local> =
    chrono::DateTime::from_utc(naive, *chrono::Local::now().offset());
  datetime.format(&format).to_string()
}

pub fn created(file: std::path::PathBuf, format: String) -> String {
  let naive = chrono::NaiveDateTime::from_timestamp(
    filetime::FileTime::from_creation_time(&file.symlink_metadata().unwrap())
      .unwrap()
      .seconds() as i64,
    0,
  );

  let datetime: chrono::DateTime<chrono::Local> =
    chrono::DateTime::from_utc(naive, *chrono::Local::now().offset());
  datetime.format(&format).to_string()
}
