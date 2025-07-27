use anyhow::{ensure, Ok, Result};
use bytemuck::{AnyBitPattern, NoUninit, Pod, Zeroable};
use lz4::block::CompressionMode;
use static_assertions::const_assert_eq;

const FILE_MARKER_LITE: [u8; 4] = *b"DVPL";

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum CompressorType {
  None,
  Lz4,
  Lz4Hc,
  Rfc1951
}

unsafe impl Zeroable for CompressorType {}
unsafe impl Pod for CompressorType {}

#[repr(C)]
#[derive(Clone, Copy, AnyBitPattern, NoUninit)]
pub struct DvplFooter {
  pub size_uncompressed: u32,
  pub size_compressed: u32,
  pub crc32_compressed: u32,
  pub compressor_type: CompressorType,
  pub pack_marker_lite: [u8; 4]
}

const_assert_eq!(std::mem::size_of::<DvplFooter>(), 20);

pub fn compress(contents: &[u8]) -> Result<Vec<u8>> {
  let (compressor_type, mut compressed) = {
    let compressed = lz4::block::compress(contents, Some(CompressionMode::HIGHCOMPRESSION(0)), false)?;

    if compressed.len() >= contents.len() {
      (CompressorType::None, Vec::from(contents))
    }
    else {
      (CompressorType::Lz4Hc, compressed)
    }
  };

  let checksum = crc32fast::hash(&compressed);

  let footer = DvplFooter {
    size_uncompressed: contents.len() as u32,
    size_compressed: compressed.len() as u32,
    crc32_compressed: checksum,
    compressor_type: compressor_type,
    pack_marker_lite: FILE_MARKER_LITE
  };

  compressed.extend_from_slice(bytemuck::bytes_of(&footer));

  Ok(compressed)
}

pub fn decompress(contents: &[u8]) -> Result<Vec<u8>> {
  ensure!(contents.len() >= std::mem::size_of::<DvplFooter>(), "not a valid dvpl file");

  let data = &contents[contents.len() - 20..];
  let footer = bytemuck::try_pod_read_unaligned::<DvplFooter>(&data).unwrap();

  ensure!(footer.pack_marker_lite == FILE_MARKER_LITE, "not a valid dvpl file");
  
  let data = &contents[..contents.len() - 20];
  let checksum = crc32fast::hash(&data);

  ensure!(footer.size_compressed == data.len() as u32 && footer.crc32_compressed == checksum, "the file is corrupted");

  let decompressed = match footer.compressor_type {
    CompressorType::None => {
      Vec::from(data)
    },
    CompressorType::Lz4 | CompressorType::Lz4Hc => {
      lz4::block::decompress(&data, Some(footer.size_uncompressed as i32))?
    },
    CompressorType::Rfc1951 => {
      inflate::inflate_bytes_zlib(&data).unwrap()
    }
  };

  Ok(decompressed)
}