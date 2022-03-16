use std::path::Path;
use std::thread;
use clap::{ArgEnum, Parser};
use digest_producer::DigestProducer;
use crate::collision_finder::CollisionFinder;
use crate::digest_producer::{DigestProducerFactory, DigestProducerHolder};

mod digest_producer;
mod crc;
mod crypto;
mod collision_finder;

#[derive(Debug, Parser)]
#[clap(version)]
struct Opts {
  dictionary: String,

  #[clap(short, long, arg_enum)]
  algorithms: Vec<Algorithm>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone, ArgEnum)]
pub enum Algorithm {
  CRC16,
  CRC32,
  CRC64,
  MD5,
  SHA1,
  SHA2,
}

fn main() {
  let opts = Opts::parse();

  let (tx, rx) = crossbeam_channel::unbounded();
  thread::spawn(move || {
    while let Ok(collision) = rx.recv() {
      eprintln!("Interim result: {collision:?}");
    }
  });

  let results = CollisionFinder::find_collisions(
    Path::new(&opts.dictionary),
    resolve_algorithms(opts.algorithms),
    tx);

  println!("Collisions by algorithm:");
  for (k, v) in &results.num_by_algorithm {
    println!(
      "  {k:?}: {} total collided hashes, {} words",
      v.total_collided_hashes,
      v.total_collided_words);
  }

  println!("All collisions");
  for collision in results.all_collisions {
    println!("{collision:?}");
  }
}

fn resolve_algorithms(opt_algorithms: Vec<Algorithm>) -> Vec<DigestProducerHolder> {
  let resolved_choices = if opt_algorithms.is_empty() {
    vec![
      Algorithm::CRC32,
      Algorithm::CRC64,
      Algorithm::MD5,
    ]
  } else {
    opt_algorithms
  };

  let factory = DigestProducerFactory::default();
  resolved_choices.into_iter()
      .map(|a| factory.create(a))
      .collect()
}
