use std::collections::HashMap;

use enum_map::EnumMap;
use lerp_table::{Piecewise, PiecewiseErr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::enums::{DefaultStat, StatHashes, StatHashesDefault};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct FiringData {
    ///The delay in frames counted at 30 fps
    pub firing_delay: u32,

    #[serde(default)]
    pub burst: Option<Burst>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Burst {
    ///The amount of frames between bullets in the same trigger pull.
    /// Measured at 30 FPS
    /// example: the bullets in a pulse rifle burst
    /// may change this to a u32
    pub burst_delay: f64,
    ///How many shots fired from one trigger pull
    pub burst_size: u32,
    ///This is true if the burst_size is more than 1, but only takes 1 ammo in the mag.
    /// example: trinity ghoul
    #[serde(default)]
    pub one_ammo: bool,
}

#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub struct StatLerp<T: StatHashesDefault> {
    ///The stat to use as input for the piecewise
    pub stat: DefaultStat<T>,

    pub lerp: Piecewise,
}

pub type Stats = EnumMap<StatHashes, Option<u32>>;
#[derive(Debug, Error)]
pub enum StatLerpErr {
    #[error(transparent)]
    PiecewiseErr(#[from] PiecewiseErr),
    #[error("Stat does not exist in provided stat map")]
    StatNoExist,
}

impl<T: StatHashesDefault + Copy> StatLerp<T> {
    pub fn solve(&self, stats: &Stats) -> Result<f64, StatLerpErr> {
        let stat_value = stats[self.stat.into()].ok_or(StatLerpErr::StatNoExist)?;
        Ok(self.lerp.y_at_x(stat_value as f64)?)
    }
}
