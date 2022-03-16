use std::fs;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use crossbeam_channel::Sender;
use crate::Algorithm;
use crate::digest_producer::DigestProducerHolder;

use rayon::prelude::*;

pub struct CollisionFinder {
  dictionary: PathBuf,
  digest_producers: Vec<DigestProducerHolder>,
  progress_tx: Option<Sender<Collision>>,
  output: Option<CollisionReport>,
}

impl CollisionFinder {
  pub fn find_collisions<P: AsRef<Path>>(
    dictionary: P,
    digest_producers: Vec<DigestProducerHolder>,
    progress_tx: Option<Sender<Collision>>,
  ) -> CollisionReport {
    let mut finder = CollisionFinder {
      dictionary: dictionary.as_ref().to_owned(),
      digest_producers,
      progress_tx,
      output: None,
    };
    finder.do_start();
    finder.output.unwrap()
  }

  fn do_start(&mut self) {
    let file = fs::File::open(&self.dictionary).unwrap();
    let by_hash = self.multiple_occurrences_by_hash(file);
    let num_by_algorithm = Self::group_by_algorithm(&by_hash);
    let mut all_collisions = Self::convert_to_collisions(by_hash);
    all_collisions.sort_unstable_by_key(|c| c.words.len());

    self.output = Some(CollisionReport {
      num_by_algorithm,
      all_collisions,
    });
  }

  fn group_by_algorithm(by_hash: &OccurrencesItem) -> HashMap<Algorithm, NumCollisions> {
    let words_by_algorithm: ByAlgorithmItem = by_hash.par_iter()
        // Map to { algorithm -> unique words, hash }
        .map(|(k, v)| {
          let mut map = HashMap::new();
          let hashes = [k.hash.clone()].into_iter().collect();
          let words = v.words.iter().cloned().collect();
          map.insert(k.algorithm, ByAlgorithmValue { hashes, words });
          map
        })
        // Reduce to { algorithm -> unique words, unique hashes }
        .reduce(HashMap::new, |i1: ByAlgorithmItem, i2: ByAlgorithmItem| {
          i2.iter().fold(i1, |mut acc: ByAlgorithmItem, (k2, v2)| {
            let v1 = acc.entry(*k2).or_default();
            v1.hashes.extend(v2.hashes.iter().cloned());
            v1.words.extend(v2.words.iter().cloned());
            acc
          })
        });
    words_by_algorithm.into_iter()
        .map(|(k, v)| {
          (k, NumCollisions {
            total_collided_hashes: v.hashes.len(),
            total_collided_words: v.words.len(),
          })
        })
        .collect()
  }

  fn convert_to_collisions(by_hash: OccurrencesItem) -> Vec<Collision> {
    let collisions_by_word: Vec<Collision> = by_hash.into_par_iter()
        // Convert to output format
        .map(|(k, v)| {
          Collision {
            words: v.words,
            hash: k.hash,
            algorithm: k.algorithm,
          }
        })
        .collect();
    collisions_by_word
  }

  fn multiple_occurrences_by_hash(&self, file: fs::File) -> OccurrencesItem {
    let by_hash: OccurrencesItem = BufReader::new(file)
        .lines()
        .filter_map(|read_line| read_line.ok())
        .par_bridge()
        // Map to { hash -> word }
        .map(|word| self.occurrences_map_function(&word))
        // Reduce to { hash -> words }
        .reduce(HashMap::new, |i1, i2| {
          self.occurrences_reduce_function(i1, i2)
        });
    by_hash.into_iter()
        // Find only collisions
        .filter(|(_k, v)| v.words.len() >= 2)
        .collect()
  }

  fn occurrences_map_function(&self, word: &str) -> OccurrencesItem {
    let word_bytes = word.as_bytes();
    self.digest_producers.iter()
        .fold(HashMap::new(), |mut acc, a| {
          let hash = a.digest_producer.produce(word_bytes);
          let key = OccurrencesKey {
            algorithm: a.algorithm,
            hash,
          };
          let value = OccurrencesValue {
            words: vec![word.to_owned()],
          };
          acc.insert(key, value);
          acc
        })
  }

  fn occurrences_reduce_function(&self, item1: OccurrencesItem, item2: OccurrencesItem) -> OccurrencesItem {
    // Move item2 into item1 and return item1
    item2.iter().fold(item1, |mut acc, (k2, v2)| {
      let v1 = acc.entry(k2.clone()).or_insert_with(OccurrencesValue::default);

      let old_len = v1.words.len();
      v1.words.extend(v2.words.iter().cloned());

      if v1.words.len() > old_len && v1.words.len() >= 2 {
        // Doesn't matter if this fails, the return value from the function will
        // provide the complete answer.
        if let Some(progress_tx) = &self.progress_tx {
          let _ = progress_tx.send(Collision {
            words: v1.words.clone(),
            algorithm: k2.algorithm,
            hash: k2.hash.clone(),
          });
        }
      }
      acc
    })
  }
}

type ByAlgorithmItem = HashMap<ByAlgorithmKey, ByAlgorithmValue>;

type ByAlgorithmKey = Algorithm;

#[derive(Debug, Default)]
struct ByAlgorithmValue {
  words: HashSet<String>,
  hashes: HashSet<Vec<u8>>,
}

type OccurrencesItem = HashMap<OccurrencesKey, OccurrencesValue>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct OccurrencesKey {
  algorithm: Algorithm,
  hash: Vec<u8>,
}

#[derive(Debug, Default)]
struct OccurrencesValue {
  words: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Collision {
  pub words: Vec<String>,
  pub algorithm: Algorithm,
  pub hash: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct CollisionReport {
  pub num_by_algorithm: HashMap<Algorithm, NumCollisions>,
  pub all_collisions: Vec<Collision>,
}

#[derive(Debug, Clone)]
pub struct NumCollisions {
  pub total_collided_hashes: usize,
  pub total_collided_words: usize,
}