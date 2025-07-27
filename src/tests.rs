use crate::dvpl;

const ORIGINAL: &[u8] = include_bytes!("../tests/quality.yaml");
const COMPRESSED: &[u8] = include_bytes!("../tests/quality.yaml.dvpl");

#[test]
fn decompress() {
  assert_eq!(dvpl::decompress(COMPRESSED).unwrap(), ORIGINAL);
}

#[test]
fn compress() {
  assert_eq!(dvpl::compress(ORIGINAL).unwrap(), COMPRESSED);
}