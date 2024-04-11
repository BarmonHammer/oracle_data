use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]

pub enum ReserveData {
    Unique([u32; 4]),
    Mapping(MappedReserves),
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct MappedReserves {
    pub mapping: Vec<f64>,
    pub ratio: f64,
}

impl MappedReserves {
    pub fn solve(&self, mag_stat: u32, inv_stat: u32) -> u32 {
        let start = self.mapping[mag_stat as usize];
        let end = start * self.ratio;
        let slope = (end - start) / 100.0;
        (inv_stat as f64 * slope + start).ceil() as u32
    }
}

impl ReserveData {
    pub fn solve(&self, reserve_count: u32, mag_stat: u32, inv_stat: u32) -> u32 {
        match self {
            Self::Unique(x) => x[reserve_count as usize],
            Self::Mapping(x) => x.solve(mag_stat, inv_stat),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum MagData {
    Single(u32),
    Mapped(Vec<u32>),
}

impl MagData {
    pub fn solve(&self, mag_stat: u32) -> u32 {
        match self {
            Self::Single(x) => *x,
            Self::Mapped(x) => x[mag_stat as usize],
        }
    }
}
