use std::time::Duration;

use crate::mouse::Vec2;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum SightID {
    None,
    Holo,
    Handmade,
    X8,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum BarrellID {
    None,
    Silencer,
}

impl BarrellID {
    pub(crate) fn get_modifier(&self) -> f32 {
        match self {
            BarrellID::None => 1f32,
            BarrellID::Silencer => 0.8f32,
        }
    }
}

impl SightID {
    pub(crate) fn get_modifier(&self) -> f32 {
        match self {
            SightID::None => 1f32,
            SightID::Holo => 1.2f32,
            SightID::Handmade => 0.8f32,
            SightID::X8 => 4.76f32,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum WeaponID {
    M2,
    HmLmg,
    Ak47,
    Lr300,
    Mp5,
    Thompson,
    Custom,
    Semi,
    Python,
}

pub(crate) struct Weapon {
    pub(crate) recoil_pattern: Vec<Vec2<f32>>,
    pub(crate) delay: Duration,
    pub(crate) weapon_id: WeaponID,
}

impl Weapon {
    pub(crate) fn new(recoil_pattern: Vec<Vec2<f32>>, delay: Duration, id: WeaponID) -> Self {
        Self {
            recoil_pattern,
            delay,
            weapon_id: id,
        }
    }
}

pub(crate) struct Settings {
    pub(crate) sensitivity: f32,
    pub(crate) weapon: WeaponID,
    pub(crate) sight: SightID,
    pub(crate) barrell: BarrellID,
}

impl Settings {
    pub(crate) fn new(sens: f32, wep: WeaponID, sig: SightID, bar: BarrellID) -> Self {
        Self {
            sensitivity: sens,
            weapon: wep,
            sight: sig,
            barrell: bar,
        }
    }
}
