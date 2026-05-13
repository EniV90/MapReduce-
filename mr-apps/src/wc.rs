use crate::MapReduceApp;
use mr_core::KeyValue;

pub struct WordCount;

impl MapReduceApp for WordCount {
    fn map(&self, _filename: &str, contents: &str) -> Vec<KeyValue> {
        contents
            .split(|c: char| !c.is_alphabetic())
            .filter(|w| !w.is_empty())
            .map(|w| KeyValue {
                key: w.to_lowercase(),
                value: "1".to_string(),
            })
            .collect()
    }

    fn reduce(&self, _key: &str, value: &[String]) -> String {
        value.len().to_string()
    }
}
