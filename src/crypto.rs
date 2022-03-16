use std::marker::PhantomData;

use digest::Digest;

use crate::DigestProducer;

#[derive(Default)]
pub struct CryptoDigestProducer<D: Digest> {
  _marker: PhantomData<D>,
}

impl<D: Digest> DigestProducer for CryptoDigestProducer<D> {
  fn produce(&self, data: &[u8]) -> Vec<u8> {
    let mut hasher = D::new();
    hasher.update(data);
    hasher.finalize().to_vec()
  }
}
