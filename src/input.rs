use structopt::StructOpt;

/// The ls replacement you never knew you needed
///
/// Written by: William Lane
#[derive(StructOpt)]
pub struct Cli {
  /// Give me a directory
  #[structopt(parse(from_os_str), default_value = ".")]
  pub dir: std::path::PathBuf,

  /// Sorts files by name
  #[structopt(short = "n", long = "name")]
  pub name: bool,

  /// Sorts files by the date created
  #[structopt(short = "c", long = "created")]
  pub created: bool,

  /// Sorts files by the date modified
  #[structopt(short = "m", long = "modified")]
  pub modified: bool,

  /// Sorts files by file size
  #[structopt(short = "s", long = "size")]
  pub size: bool,

  /// Groups directorys before files
  #[structopt(short = "g", long = "gdf")]
  pub gdf: bool,

  /// Enables long mode (permissions, size, user, group)
  #[structopt(short = "l", long = "long")]
  pub long: bool,

  /// Formats the time output 
  #[structopt(long = "time-format", default_value = "%e %b %H.%M")]
  pub time_format: String,

  /// Shows the file created time instead of the file modified time
  #[structopt(short = "i", long = "ct")]
  pub created_time: bool,
}
