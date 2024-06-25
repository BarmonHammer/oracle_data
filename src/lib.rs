use formula_structs::firing_data::StatLerp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};
use thiserror::Error;
use tracing::{span, Level};

pub mod enums;
pub mod formula_structs;

pub type BungieHash = u32;
pub type WeaponType = u32;
pub type Time = u64;

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub struct WeaponPath {
    ///Destiny.DestinyItemSubType from bungie api
    pub weapon_type: WeaponType,
    ///Can be weapon hash or intrinsic hash
    pub hash: BungieHash,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct MappedData<T> {
    data: Vec<T>,
    map: HashMap<WeaponPath, (usize, Time)>,
}

pub struct WeaponFormula<'a, T> {
    pub formula: &'a T,
    pub timestamp: Time,
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, Default)]
pub struct Inner<T> {
    pub formula: T,
    pub weapons: Vec<WeaponPath>,
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, Default)]
pub struct Outer<T> {
    pub data: Vec<Inner<T>>,
}

#[derive(Debug, Error)]
//name pending
pub enum DataError {
    #[error("Weapon entry {0:?} already exists")]
    DoubleEntry(WeaponPath),
}

fn get_unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn make_diff<T: Clone + PartialEq>(
    new: Outer<T>,
    old: MappedData<T>,
) -> Result<MappedData<T>, DataError> {
    let mut map: HashMap<WeaponPath, (usize, Time)> = HashMap::new();
    let mut data: Vec<T> = Vec::new();

    for (index, group) in new.data.iter().cloned().enumerate() {
        for path in group.weapons {
            let timestamp = match old.get(path.hash, path.hash, path.weapon_type) {
                Some(old_formula) if old_formula.formula == &group.formula => old_formula.timestamp,
                _ => get_unix_time(),
            };

            if map.insert(path, (index, timestamp)).is_some() {
                return Err(DataError::DoubleEntry(path));
            }
        }

        data.push(group.formula);
    }

    Ok(MappedData { data, map })
}

impl<T> MappedData<T> {
    ///Checks for weapon hash first, then intrinsic
    pub fn get(
        &self,
        weapon_hash: u32,
        intrinsic_hash: u32,
        weapon_type: u32,
    ) -> Option<WeaponFormula<T>> {
        if let Some(data) = self.map.get(&WeaponPath {
            weapon_type,
            hash: weapon_hash,
        }) {
            return Some(WeaponFormula {
                formula: &self.data[data.0],
                timestamp: data.1,
            });
        }
        let data = self.map.get(&WeaponPath {
            weapon_type,
            hash: intrinsic_hash,
        })?;
        Some(WeaponFormula {
            formula: &self.data[data.0],
            timestamp: data.1,
        })
    }
}
