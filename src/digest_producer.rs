use std::collections::hash_map::DefaultHasher;
use fasthash::{MetroHasher, SpookyHasher};
use crate::Algorithm;
use crate::crc::{Crc16DigestProducer, Crc32DigestProducer, Crc64DigestProducer};
use crate::crypto::CryptoDigestProducer;
use crate::hasher::HasherDigestProducer;

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
      Algorithm::StdDefault => Box::new(HasherDigestProducer::<DefaultHasher>::default()),
      Algorithm::Metro64 => Box::new(HasherDigestProducer::<MetroHasher>::default()),
      Algorithm::Spooky64 => Box::new(HasherDigestProducer::<SpookyHasher>::default()),
      Algorithm::Crc16 => Box::new(Crc16DigestProducer::default()),
      Algorithm::Crc32 => Box::new(Crc32DigestProducer::default()),
      Algorithm::Crc64 => Box::new(Crc64DigestProducer::default()),
      Algorithm::Md5 => Box::new(CryptoDigestProducer::<md5::Md5>::default()),
      Algorithm::Sha1 => Box::new(CryptoDigestProducer::<sha1::Sha1>::default()),
      Algorithm::Sha2 => Box::new(CryptoDigestProducer::<sha2::Sha256>::default()),
    };

    DigestProducerHolder {
      algorithm,
      digest_producer,
    }
  }
}