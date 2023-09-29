#[derive(Debug)]
pub enum Device {
    Factory { multiplier: f32 },
    Farm { multiplier: f32 },
    MissileSilo,
    Mine { multiplier: f32 },
    PowerPlant { multiplier: f32 },
    Satellite,
}

impl Device {
    pub fn lsname(&self) -> &'static str {
        use Device::*;
        match self {
            Factory { .. } => "factory",
            Farm { .. } => "farm",
            MissileSilo => "missile-silo",
            Mine { .. } => "mine",
            PowerPlant { .. } => "power-plant",
            Satellite => "satellite",
        }
    }
}
