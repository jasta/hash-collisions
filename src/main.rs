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

  #[clap(short, long, arg_enum, default_value_t = Format::SUMMARY)]
  format: Format,

  #[clap(short, long)]
  progress: bool,
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

#[derive(Debug, Eq, PartialEq, Clone, ArgEnum)]
pub enum Format {
  SUMMARY,
  WORDS,
  BOTH,
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

  if opts.format == Format::SUMMARY || opts.format == Format::BOTH {
    println!("Collisions by algorithm:");
    for (k, v) in &results.num_by_algorithm {
      println!(
        "  {k:?}: {} total collided hashes, {} words",
        v.total_collided_hashes,
        v.total_collided_words);
    }
  }

  if opts.format == Format::WORDS || opts.format == Format::BOTH {
    println!("All collisions");
    for collision in results.all_collisions {
      println!("{collision:?}");
    }
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
