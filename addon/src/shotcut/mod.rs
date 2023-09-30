use lnk_parser::LNKParser;
use std::{fs::File, path::PathBuf};
pub mod exelook;

#[napi]
#[cfg(any(target_os = "windows"))]
pub fn parse_lnk(path: String) -> Option<String> {
  let mut file = File::open(path).unwrap();
  let lnk_file = LNKParser::from_reader(&mut file);
  if let Ok(f) = lnk_file {
    Some(serde_json::to_string(&f).unwrap())
  } else {
    None
  }
}

#[napi]
pub struct LnkData {
  pub name_string: Option<String>,
  pub relative_path: Option<String>,
  pub working_dir: Option<String>,
  pub icon_location: Option<String>,
}

fn convert(p: Option<PathBuf>) -> Option<String> {
  match p {
    Some(p) => Some(p.to_str().unwrap().to_string()),
    None => None,
  }
}

#[napi]
#[cfg(any(target_os = "windows"))]
pub fn parse_lnk_fallback(path: String) -> LnkData {
  let lnk_path = std::path::Path::new(&path);
  let lnk = parselnk::Lnk::try_from(lnk_path).unwrap();

  LnkData {
    name_string: lnk.string_data.name_string,
    relative_path: convert(lnk.string_data.relative_path),
    working_dir: convert(lnk.string_data.working_dir),
    icon_location: convert(lnk.string_data.icon_location),
  }
}
