pub mod indexer;
pub mod wc;

use mr_core::KeyValue;

pub trait MapReduceApp: Send + Sync + 'static {
    fn map(&self, filename: &str, contents: &str) -> Vec<KeyValue>;
    fn reduce(&self, key: &str, value: &[String]) -> String;
}
