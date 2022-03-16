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
mod hasher;

#[derive(Debug, Parser)]
#[clap(version)]
struct Opts {
  dictionary: String,

  #[clap(short, long, arg_enum)]
  algorithms: Vec<Algorithm>,

  #[clap(short, long, arg_enum, default_value_t = Format::Summary)]
  format: Format,

  #[clap(short, long)]
  progress: bool,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone, ArgEnum)]
pub enum Algorithm {
  StdDefault,
  Metro64,
  Spooky64,
  Crc16,
  Crc32,
  Crc64,
  Md5,
  Sha1,
  Sha2,
}

#[derive(Debug, Eq, PartialEq, Clone, ArgEnum)]
pub enum Format {
  Summary,
  Words,
  Both,
}

fn main() {
  let opts = Opts::parse();

  let tx = if opts.progress {
    let (tx, rx) = crossbeam_channel::unbounded();
    thread::spawn(move || {
      while let Ok(collision) = rx.recv() {
        eprintln!("Interim result: {collision:?}");
      }
    });
    Some(tx)
  } else {
    None
  };

  let results = CollisionFinder::find_collisions(
    Path::new(&opts.dictionary),
    resolve_algorithms(opts.algorithms),
    tx);

  if opts.format == Format::Summary || opts.format == Format::Both {
    println!("Collisions by algorithm:");
    for (k, v) in &results.num_by_algorithm {
      println!(
        "  {k:?}: {} total collided hashes, {} words",
        v.total_collided_hashes,
        v.total_collided_words);
    }
  }

  if opts.format == Format::Words || opts.format == Format::Both {
    println!("All collisions");
    for collision in results.all_collisions {
      println!("{collision:?}");
    }
  }
}

fn resolve_algorithms(opt_algorithms: Vec<Algorithm>) -> Vec<DigestProducerHolder> {
  let resolved_choices = if opt_algorithms.is_empty() {
    vec![
      Algorithm::StdDefault,
      Algorithm::Metro64,
      Algorithm::Spooky64,
      Algorithm::Crc32,
      Algorithm::Crc64,
      Algorithm::Md5,
    ]
  } else {
    opt_algorithms
  };

  let factory = DigestProducerFactory::default();
  resolved_choices.into_iter()
      .map(|a| factory.create(a))
      .collect()
}
