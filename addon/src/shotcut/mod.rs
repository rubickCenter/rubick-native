use lnk_parser::LNKParser;
use napi::Result;
use std::path::{absolute, PathBuf};
pub mod exelook;

#[napi]
pub fn parse_lnk(path: String) -> Result<LnkData> {
  let lnk_file = LNKParser::from_path(&path);
  match lnk_file {
    Ok(f) => {
      let name_string = f.get_name_string().as_ref().map(|f| f.to_string());
      let full_path = f
        .get_target_full_path()
        .as_ref()
        .map(|f| {
          if f.starts_with("MY_COMPUTER\\") {
            Some(f.to_string().replace("MY_COMPUTER\\", ""))
          } else {
            Some(f.to_string())
          }
        })
        .map_or(None, |f| f);
      let working_dir = f.get_working_dir().as_ref().map(|f| f.to_string());
      let icon_location = f.get_icon_location().as_ref().map(|f| f.to_string());

      Ok(LnkData {
        name_string,
        full_path,
        working_dir,
        icon_location,
      })
    }
    Err(_) => {
      let lnk_path = std::path::Path::new(&path);
      let lnk = parselnk::Lnk::try_from(lnk_path);
      match lnk {
        Ok(l) => {
          let s = absolute(
            PathBuf::from(lnk_path)
              .parent()
              .unwrap()
              .join(l.string_data.relative_path.unwrap()),
          )
          .map_or(None, |f| Some(f));

          Ok(LnkData {
            name_string: l.string_data.name_string,
            full_path: convert(s),
            working_dir: convert(l.string_data.working_dir),
            icon_location: convert(l.string_data.icon_location),
          })
        }
        Err(e) => Err(napi::Error::from_reason(e.to_string())),
      }
    }
  }
}

#[napi]
pub struct LnkData {
  pub name_string: Option<String>,
  pub full_path: Option<String>,
  pub working_dir: Option<String>,
  pub icon_location: Option<String>,
}

fn convert(p: Option<PathBuf>) -> Option<String> {
  p.map(|f| f.to_string_lossy().to_string())
}
