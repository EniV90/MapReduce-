use mr_core::KeyValue;

use crate::MapReduceApp;

pub struct Indexer;

impl MapReduceApp for Indexer {
    fn map(&self, filename: &str, contents: &str) -> Vec<KeyValue> {
        contents
            .split(|c: char| !c.is_alphabetic())
            .filter(|w| !w.is_empty())
            .map(|w| KeyValue {
                key: w.to_lowercase(),
                value: filename.to_string(),
            })
            .collect()
    }

    fn reduce(&self, _key: &str, value: &[String]) -> String {
        let mut filenames = value.to_vec();
        filenames.sort();
        filenames.join(",")
    }
}
