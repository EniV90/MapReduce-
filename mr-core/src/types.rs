use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyValue {
    // Data in this KeyVlaue Flows through the whole system
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskKind {
    // The coordiator tells a worker what to do
    Map {
        id: usize, // worker needs to know it's task Id to it knows to write to mr-{id}-0, mr-{id}-1
        filename: String, // work needs to know which input file to read
        n_reduce: usize, // worker needs to know how many reduced bucket exists so it knows how many output files to create and how to route keys
    },
    Reduce {
        id: usize,    // worker needs to know which bucket it owns so it reads mr-0-{id}, mr-1-{id}
        n_map: usize, // worker needs to know how many map task ran "n_mapsize: usize", so it knows how many mr-X-{id} file to look for
    },
    Wait, // doesn't do much, just a signal: No task ready, sleep and retry
    Exit, // doesn't do much, just a signal: job done, terminate
}

#[derive(Debug, Clone)]
pub enum TaskState {
    // Coordinator tracks each task internally with this
    Idle,
    Inprogress(Instant),
    Completed,
}
