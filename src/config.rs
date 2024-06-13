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

pub(crate) struct Settings {
    pub(crate) sensitivity: f32,
    pub(crate) weapon: WeaponID,
}

impl Settings {
    pub(crate) fn new(sens: f32, wep: WeaponID) -> Self {
        Self {
            sensitivity: sens,
            weapon: wep,
        }
    }
}
