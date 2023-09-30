// 借用该项目源码(Apache-2.0 license): https://github.com/Lucius-Q-User/ExeLook
mod dib;

use base64::{engine::general_purpose, Engine};

use pelite::Error::Bounds;
use pelite::{
  self,
  resources::{FindError, Resources},
  FileMap, PeFile,
};

use std::{
  convert::{From, TryInto},
  io,
  str::Utf8Error,
};

impl<'a> BitmapInfoHeader<'a> {
  pub(crate) fn from_bytes(bytes: &[u8]) -> Result<BitmapInfoHeader> {
    if bytes.len() >= 40 {
      Ok(BitmapInfoHeader {
        bytes: &bytes[..40],
      })
    } else {
      Err(Bounds.into())
    }
  }
  pub(crate) fn width(&self) -> i32 {
    i32::from_le_bytes(self.bytes[4..8].try_into().unwrap())
  }
  pub(crate) fn height(&self) -> i32 {
    i32::from_le_bytes(self.bytes[8..12].try_into().unwrap())
  }
  pub(crate) fn planes(&self) -> u16 {
    u16::from_le_bytes(self.bytes[12..14].try_into().unwrap())
  }
  pub(crate) fn bit_count(&self) -> u16 {
    u16::from_le_bytes(self.bytes[14..16].try_into().unwrap())
  }
  pub(crate) fn compression(&self) -> u32 {
    u32::from_le_bytes(self.bytes[16..20].try_into().unwrap())
  }
}

struct BitmapInfoHeader<'a> {
  bytes: &'a [u8],
}

#[derive(Debug)]
pub enum Error {
  Io(io::Error),
  Pe(pelite::Error),
  UTF(Utf8Error),
  NoIconFound,
  PlanarNotSupported,
  UnrecognizedBPP,
  UnknownCompression,
  MalformedPng,
}

impl From<Utf8Error> for Error {
  fn from(err: Utf8Error) -> Self {
    Error::UTF(err)
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Error::Io(err)
  }
}

impl From<pelite::Error> for Error {
  fn from(err: pelite::Error) -> Self {
    Error::Pe(err)
  }
}

impl From<FindError> for Error {
  fn from(_err: FindError) -> Self {
    Error::NoIconFound
  }
}

struct PngHeader<'a> {
  bytes: &'a [u8],
}

impl<'a> PngHeader<'a> {
  fn from_bytes<'b>(bytes: &'b [u8]) -> Result<PngHeader<'b>> {
    if bytes.len() < 24 || bytes[12..16] != [b'I', b'H', b'D', b'R'] {
      Err(Error::MalformedPng)
    } else {
      Ok(PngHeader { bytes })
    }
  }
  fn width(&self) -> i32 {
    u32::from_be_bytes(self.bytes[16..20].try_into().unwrap()) as i32
  }
  fn height(&self) -> i32 {
    u32::from_be_bytes(self.bytes[20..24].try_into().unwrap()) as i32
  }
}

pub type Result<T> = ::std::result::Result<T, Error>;

fn get_resources(bytes: &[u8]) -> Result<Resources> {
  let res = PeFile::from_bytes(bytes)?.resources();
  if let Err(pelite::Error::Null) = res {
    Err(Error::NoIconFound)
  } else {
    res.map_err(Into::into)
  }
}

fn is_png(bytes: &[u8]) -> bool {
  bytes.starts_with(&[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a])
}

fn icon_compare_key(icon: &[u8]) -> Result<impl PartialOrd> {
  Ok(if is_png(icon) {
    let hdr = PngHeader::from_bytes(icon)?;
    (hdr.width(), hdr.height(), 64)
  } else {
    let hdr = BitmapInfoHeader::from_bytes(icon)?;
    (hdr.width(), hdr.height(), hdr.bit_count())
  })
}

fn best_icon<'a>(mut icons: impl Iterator<Item = Result<&'a [u8]>>) -> Result<&'a [u8]> {
  let mut cur_max: &'a [u8] = if let Some(x) = icons.next() {
    x?
  } else {
    return Err(Error::NoIconFound);
  };
  let mut cur_max_key = icon_compare_key(cur_max)?;
  for icon in icons {
    let icon = icon?;
    let key = icon_compare_key(icon)?;
    if key > cur_max_key {
      cur_max = icon;
      cur_max_key = key;
    }
  }
  Ok(cur_max)
}

#[napi]
pub struct ShorCutImg {
  pub data: Vec<u8>,
  pub width: i32,
  pub height: i32,
}

fn _exelook(file_name: String) -> Result<ShorCutImg> {
  let map_region = FileMap::open(&file_name)?;
  let resources = get_resources(map_region.as_ref())?;
  let (_, icon_group) = resources.icons().next().ok_or(Error::NoIconFound)??;
  let icons = icon_group
    .entries()
    .iter()
    .map(|ent| icon_group.image(ent.nId).map_err(Into::into));

  let best_icon = best_icon(icons)?;
  if is_png(best_icon) {
    Ok(ShorCutImg {
      data: best_icon.to_owned(),
      width: 0,
      height: 0,
    })
  } else {
    let infoheader = BitmapInfoHeader::from_bytes(best_icon)?;
    if infoheader.planes() != 1 {
      return Err(Error::PlanarNotSupported);
    }
    if infoheader.compression() != 0 {
      return Err(Error::UnknownCompression);
    }
    let data = dib::decode_dib(best_icon)?;

    Ok(ShorCutImg {
      data,
      width: infoheader.width(),
      height: infoheader.height() / 2,
    })
  }
}

#[napi]
#[cfg(any(target_os = "windows"))]
pub fn exe_look_base64(file_name: String) -> Option<String> {
  if let Ok(l) = _exelook(file_name) {
    Some("data:image/*;base64,".to_owned() + &general_purpose::STANDARD.encode(l.data))
  } else {
    None
  }
}
