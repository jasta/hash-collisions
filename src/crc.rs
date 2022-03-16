use crc::Crc;
use crate::DigestProducer;

#[derive(Default)]
pub struct Crc16DigestProducer;
impl DigestProducer for Crc16DigestProducer {
  fn produce(&self, data: &[u8]) -> Vec<u8> {
    Crc::<u16>::new(&crc::CRC_16_IBM_SDLC).checksum(data).to_ne_bytes().to_vec()
  }
}

#[derive(Default)]
pub struct Crc32DigestProducer;
impl DigestProducer for Crc32DigestProducer {
  fn produce(&self, data: &[u8]) -> Vec<u8> {
    Crc::<u32>::new(&crc::CRC_32_ISCSI).checksum(data).to_ne_bytes().to_vec()
  }
}

#[derive(Default)]
pub struct Crc64DigestProducer;
impl DigestProducer for Crc64DigestProducer {
  fn produce(&self, data: &[u8]) -> Vec<u8> {
    Crc::<u64>::new(&crc::CRC_64_ECMA_182).checksum(data).to_ne_bytes().to_vec()
  }
}
