use crate::Algorithm;
use crate::crc::{Crc16DigestProducer, Crc32DigestProducer, Crc64DigestProducer};
use crate::crypto::CryptoDigestProducer;

pub struct DigestProducerHolder {
  pub algorithm: Algorithm,
  pub digest_producer: Box<dyn DigestProducer + Send + Sync>,
}

pub trait DigestProducer {
  fn produce(&self, data: &[u8]) -> Vec<u8>;
}

#[derive(Default)]
pub struct DigestProducerFactory;

impl DigestProducerFactory {
  pub fn create(&self, algorithm: Algorithm) -> DigestProducerHolder {
    let digest_producer: Box<dyn DigestProducer + Send + Sync> = match algorithm {
      Algorithm::CRC16 => Box::new(Crc16DigestProducer::default()),
      Algorithm::CRC32 => Box::new(Crc32DigestProducer::default()),
      Algorithm::CRC64 => Box::new(Crc64DigestProducer::default()),
      Algorithm::MD5 => Box::new(CryptoDigestProducer::<md5::Md5>::default()),
      Algorithm::SHA1 => Box::new(CryptoDigestProducer::<sha1::Sha1>::default()),
      Algorithm::SHA2 => Box::new(CryptoDigestProducer::<sha2::Sha256>::default()),
    };

    DigestProducerHolder {
      algorithm,
      digest_producer,
    }
  }
}