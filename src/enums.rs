use std::marker::PhantomData;

use default_enum::EnumVariants;
use enum_map::{Enum, EnumMap};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    TryFromPrimitive,
    IntoPrimitive,
    Serialize,
    Deserialize,
    JsonSchema,
    Hash,
    Debug,
)]
#[repr(u32)]
#[derive(Enum, EnumVariants)]
pub enum StatHashes {
    Accuracy = 1591432999,
    AimAssistance = 1345609583,
    AirborneEffectiveness = 2714457168,
    AmmoCapacity = 925767036,
    Attack = 1480404414,
    BlastRadius = 3614673599,
    ChargeRate = 3022301683,
    ChargeTime = 2961396640,
    Discipline = 1735777505,
    DrawTime = 447667954,
    GuardEfficiency = 2762071195,
    GuardEndurance = 3736848092,
    GuardResistance = 209426660,
    Handling = 943549884,
    Impact = 4043523819,
    Intellect = 144602215,
    InventorySize = 1931675084,
    Magazine = 3871231066,
    Mobility = 2996146975,
    Power = 1935470627,
    Range = 1240592695,
    RecoilDirection = 2715839340,
    Recovery = 1943323491,
    Reload = 4188031367,
    Resilience = 392767087,
    RoundsPerMinute = 4284893193,
    ShieldDuration = 1842278586,
    Stability = 155624089,
    Strength = 4244567218,
    SwingSpeed = 2837207746,
    Velocity = 2523465841,
    Zoom = 3555269338,
}

#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema, Copy,
)]
pub struct DefaultStat<T>(StatHashes, PhantomData<T>);
impl<T: StatHashesDefault> Default for DefaultStat<T> {
    fn default() -> Self {
        DefaultStat(T::as_variant(), PhantomData)
    }
}

impl<T> From<DefaultStat<T>> for StatHashes {
    fn from(value: DefaultStat<T>) -> Self {
        value.0
    }
}

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use crate::enums::{DefaultStat, Handling, Reload, Stability, StatHashes, StatHashesDefault};

    #[test]
    fn test_enum() {
        assert_eq!(Stability::as_variant(), StatHashes::Stability)
    }
    #[test]
    fn test_default_enum() {
        assert_eq!(
            StatHashes::from(DefaultStat::<Stability>::default()),
            StatHashes::Stability
        );
        assert_eq!(
            StatHashes::from(DefaultStat::<Handling>::default()),
            StatHashes::Handling
        );
        assert_eq!(
            StatHashes::from(DefaultStat::<Reload>::default()),
            StatHashes::Reload
        );
    }
}
