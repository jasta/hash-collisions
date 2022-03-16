use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use crate::DigestProducer;

#[derive(Debug, Default)]
pub struct HasherDigestProducer<H: Hasher + Default> {
  _marker: PhantomData<H>,
}
impl<H: Hasher + Default> DigestProducer for HasherDigestProducer<H> {
  fn produce(&self, data: &[u8]) -> Vec<u8> {
    let mut hasher = H::default();
    data.hash(&mut hasher);
    hasher.finish().to_ne_bytes().to_vec()
  }
}