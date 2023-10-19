use lnk_parser::LNKParser;
use napi::Result;
use std::path::PathBuf;
pub mod exelook;

#[napi]
pub fn parse_lnk(path: String) -> Result<String> {
  let lnk_file = LNKParser::from_path(&path);
  match lnk_file {
    Ok(f) => Ok(serde_json::to_string(&f).unwrap()),
    Err(e) => Err(napi::Error::from_reason(e.to_string())),
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
pub fn parse_lnk_fallback(path: String) -> Result<LnkData> {
  let lnk_path = std::path::Path::new(&path);
  let lnk = parselnk::Lnk::try_from(lnk_path);
  match lnk {
    Ok(l) => Ok(LnkData {
      name_string: l.string_data.name_string,
      relative_path: convert(l.string_data.relative_path),
      working_dir: convert(l.string_data.working_dir),
      icon_location: convert(l.string_data.icon_location),
    }),
    Err(e) => Err(napi::Error::from_reason(e.to_string())),
  }
}
