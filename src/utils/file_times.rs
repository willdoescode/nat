pub fn modified(file: std::path::PathBuf, format: String) -> String {
  if file.symlink_metadata().unwrap().modified().is_ok() {
    let naive = chrono::NaiveDateTime::from_timestamp(
      filetime::FileTime::from_last_modification_time(&file.symlink_metadata().unwrap()).seconds()
      as i64,
      0,
      );

    let datetime: chrono::DateTime<chrono::Local> =
      chrono::DateTime::from_utc(naive, *chrono::Local::now().offset());
    datetime.format(&format).to_string()
  } else {
    "00 000 00:00:00".to_string()
  }
}

pub fn created(file: std::path::PathBuf, format: String) -> String {
  if filetime::FileTime::from_creation_time(&file.symlink_metadata().unwrap()).is_some() {
    let naive = chrono::NaiveDateTime::from_timestamp(
      filetime::FileTime::from_creation_time(&file.symlink_metadata().unwrap())
      .unwrap()
      .seconds() as i64,
      0,
      );

    let datetime: chrono::DateTime<chrono::Local> =
      chrono::DateTime::from_utc(naive, *chrono::Local::now().offset());
    datetime.format(&format).to_string()
  } else {
    "00 000 00:00:00".to_string()
  }
}
