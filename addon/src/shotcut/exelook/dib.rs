use super::{Error, Result};
use pelite::Error::Bounds;
use std::{convert::TryInto, fmt};

pub(crate) struct BitmapInfoHeader<'a> {
  bytes: &'a [u8],
}

#[derive(Debug)]
pub struct Pixel {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8,
}

impl Pixel {
  fn copy_to_vec(&self, vec: &mut Vec<u8>) {
    vec.push(self.red);
    vec.push(self.green);
    vec.push(self.blue);
    vec.push(self.alpha);
  }
}

pub struct DIB<'a> {
  palette: &'a [u8],
  xor_mask: &'a [u8],
  and_mask: &'a [u8],
  row_size: usize,
  mask_row_size: usize,
  real_height: usize,
  width: usize,
  bit_count: u16,
  upside_down: bool,
}

impl<'a> DIB<'a> {
  fn pixel_at_1bpp(&self, x: usize, ry: usize, rym: usize) -> Pixel {
    let idx = (self.xor_mask[ry + x / 8] >> (7 - (x % 8))) as usize & 1;
    let blue = self.palette[idx * 4];
    let green = self.palette[idx * 4 + 1];
    let red = self.palette[idx * 4 + 2];
    let alpha = (self.and_mask[rym + x / 8] >> (7 - (x % 8))) & 1;
    Pixel {
      red,
      green,
      blue,
      alpha: if alpha == 0 { 255 } else { 0 },
    }
  }
  fn pixel_at_4bpp(&self, x: usize, ry: usize, rym: usize) -> Pixel {
    let idx = (self.xor_mask[ry + x / 2] >> (if x % 2 == 0 { 4 } else { 0 })) as usize & 15;
    let blue = self.palette[idx * 4];
    let green = self.palette[idx * 4 + 1];
    let red = self.palette[idx * 4 + 2];
    let alpha = (self.and_mask[rym + x / 8] >> (7 - (x % 8))) & 1;
    Pixel {
      red,
      green,
      blue,
      alpha: if alpha == 0 { 255 } else { 0 },
    }
  }
  fn pixel_at_8bpp(&self, x: usize, ry: usize, rym: usize) -> Pixel {
    let idx = self.xor_mask[ry + x] as usize;
    let blue = self.palette[idx * 4];
    let green = self.palette[idx * 4 + 1];
    let red = self.palette[idx * 4 + 2];
    let alpha = (self.and_mask[rym + x / 8] >> (7 - (x % 8))) & 1;
    Pixel {
      red,
      green,
      blue,
      alpha: if alpha == 0 { 255 } else { 0 },
    }
  }
  fn pixel_at_24bpp(&self, x: usize, ry: usize, rym: usize) -> Pixel {
    let blue = self.xor_mask[ry + x * 3];
    let green = self.xor_mask[ry + x * 3 + 1];
    let red = self.xor_mask[ry + x * 3 + 2];
    let alpha = (self.and_mask[rym + x / 8] >> (7 - (x % 8))) & 1;
    Pixel {
      red,
      green,
      blue,
      alpha: if alpha == 0 { 255 } else { 0 },
    }
  }
  fn pixel_at_32bpp(&self, x: usize, ry: usize, rym: usize) -> Pixel {
    let blue = self.xor_mask[ry + x * 4];
    let green = self.xor_mask[ry + x * 4 + 1];
    let red = self.xor_mask[ry + x * 4 + 2];
    let alpha = self.xor_mask[ry + x * 4 + 3];
    let mask = (self.and_mask[rym + x / 8] >> (7 - (x % 8))) & 1;
    Pixel {
      red,
      green,
      blue,
      alpha: if alpha == 0 && mask == 0 { 255 } else { alpha },
    }
  }
  fn from_bytes<'b>(hdr: &'b BitmapInfoHeader, bytes: &'b [u8]) -> Result<DIB<'b>> {
    match hdr.bit_count() {
      1 => {
        let row_size = hdr.width() as usize / 8 + if hdr.width() % 8 != 0 { 1 } else { 0 };
        DIB::from_bytes_shared(hdr, bytes, 4 * 2, row_size)
      }
      4 => {
        let row_size = (hdr.width() / 2 + hdr.width() % 2) as usize;
        DIB::from_bytes_shared(hdr, bytes, 4 * 16, row_size)
      }
      8 => DIB::from_bytes_shared(hdr, bytes, 4 * 256, hdr.width() as usize),
      24 => DIB::from_bytes_shared(hdr, bytes, 0, hdr.width() as usize * 3),
      32 => DIB::from_bytes_shared(hdr, bytes, 0, hdr.width() as usize * 4),
      _ => Err(Error::UnrecognizedBPP),
    }
  }
  fn decode(&self) -> Vec<u8> {
    let mut pixels = Vec::with_capacity(self.real_height * self.width * 4);
    for y in 0..self.real_height {
      for x in 0..self.width {
        let ry = if self.upside_down {
          (self.real_height - 1 - y) * self.row_size
        } else {
          y * self.row_size
        };
        let rym = if self.upside_down {
          (self.real_height - 1 - y) * self.mask_row_size
        } else {
          y * self.mask_row_size
        };
        match self.bit_count {
          // hoping that loop unswitching will kick in here
          1 => {
            self.pixel_at_1bpp(x, ry, rym).copy_to_vec(&mut pixels);
          }
          4 => {
            self.pixel_at_4bpp(x, ry, rym).copy_to_vec(&mut pixels);
          }
          8 => {
            self.pixel_at_8bpp(x, ry, rym).copy_to_vec(&mut pixels);
          }
          24 => {
            self.pixel_at_24bpp(x, ry, rym).copy_to_vec(&mut pixels);
          }
          32 => {
            self.pixel_at_32bpp(x, ry, rym).copy_to_vec(&mut pixels);
          }
          _ => {
            unreachable!();
          }
        }
      }
    }
    pixels
  }

  fn from_bytes_shared<'b>(
    hdr: &'b BitmapInfoHeader,
    bytes: &'b [u8],
    palette_size: usize,
    mut row_size: usize,
  ) -> Result<DIB<'b>> {
    let header_size = 40;
    let image_data_offset = header_size + palette_size;
    let real_height = (hdr.height().abs() / 2) as usize;
    if row_size % 4 != 0 {
      row_size += 4 - row_size % 4;
    }
    let xor_mask_size = row_size * real_height;
    let and_mask_offset = image_data_offset + xor_mask_size;
    let mut mask_row_size =
      hdr.width() as usize / 8 + if hdr.width() as usize % 8 != 0 { 1 } else { 0 };
    if mask_row_size % 4 != 0 {
      mask_row_size += 4 - mask_row_size % 4;
    }
    let and_mask_size = mask_row_size * real_height;
    let image_end = and_mask_offset + and_mask_size;
    let and_mask = bytes
      .get(and_mask_offset..image_end)
      .ok_or_else(|| Error::from(Bounds))?;
    let palette = &bytes[header_size..image_data_offset]; // first check dominates the following checks
    let xor_mask = &bytes[image_data_offset..and_mask_offset]; // keep it above them to use only one .get
    let upside_down = hdr.height() > 0;
    Ok(DIB {
      palette,
      xor_mask,
      and_mask,
      row_size,
      mask_row_size,
      real_height,
      upside_down,
      width: hdr.width() as usize,
      bit_count: hdr.bit_count(),
    })
  }
}

pub(crate) fn decode_dib(bytes: &[u8]) -> Result<Vec<u8>> {
  let hdr = BitmapInfoHeader::from_bytes(bytes)?;
  let dib = DIB::from_bytes(&hdr, bytes)?;
  Ok(dib.decode())
}

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
  pub(crate) fn size(&self) -> u32 {
    u32::from_le_bytes(self.bytes[0..4].try_into().unwrap())
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
  pub(crate) fn image_size(&self) -> u32 {
    u32::from_le_bytes(self.bytes[20..24].try_into().unwrap())
  }
  pub(crate) fn x_px_per_meter(&self) -> i32 {
    i32::from_le_bytes(self.bytes[24..28].try_into().unwrap())
  }
  pub(crate) fn y_px_per_meter(&self) -> i32 {
    i32::from_le_bytes(self.bytes[28..32].try_into().unwrap())
  }
  pub(crate) fn colors_used(&self) -> u32 {
    u32::from_le_bytes(self.bytes[32..36].try_into().unwrap())
  }
  pub(crate) fn colors_important(&self) -> u32 {
    u32::from_le_bytes(self.bytes[36..40].try_into().unwrap())
  }
}

impl<'a> fmt::Debug for BitmapInfoHeader<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("BitmapInfoHeader")
      .field("size", &self.size())
      .field("width", &self.width())
      .field("height", &self.height())
      .field("planes", &self.planes())
      .field("bit_count", &self.bit_count())
      .field("compression", &self.compression())
      .field("image_size", &self.image_size())
      .field("x_px_per_meter", &self.x_px_per_meter())
      .field("y_px_per_meter", &self.y_px_per_meter())
      .field("colors_used", &self.colors_used())
      .field("colors_important", &self.colors_important())
      .finish()
  }
}
