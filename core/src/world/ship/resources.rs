use std::collections::HashMap;
use strum::EnumIter;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, EnumIter,
)]
pub enum ShipResource {
    Power,
    Heat,
    Protium,
    Deuterium,
    Tritium,
    Helium3,
    Helium4,
    Lithium,
    Hydrogen,
    Oxygen,
    Carbon,
    Nitrogen,
    Water,
    CarbonDioxide,
    Methane,
}

impl ShipResource {
    pub fn short_name(&self) -> &'static str {
        match self {
            ShipResource::Power => "PWR",
            ShipResource::Heat => "HEAT",
            ShipResource::Protium => "PROT",
            ShipResource::Deuterium => "DEUT",
            ShipResource::Tritium => "TRIT",
            ShipResource::Helium3 => "HE3",
            ShipResource::Helium4 => "HE4",
            ShipResource::Lithium => "LI",
            ShipResource::Hydrogen => "H",
            ShipResource::Oxygen => "O2",
            ShipResource::Carbon => "C",
            ShipResource::Nitrogen => "N",
            ShipResource::Water => "H2O",
            ShipResource::CarbonDioxide => "CO2",
            ShipResource::Methane => "CH4",
        }
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ShipResources(HashMap<ShipResource, f64>);

impl ShipResources {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, resource: &ShipResource) -> f64 {
        self.0.get(resource).copied().unwrap_or(0.0)
    }

    pub fn set(&mut self, resource: ShipResource, amount: f64) {
        self.0.insert(resource, amount);
    }

    pub fn add(&mut self, resource: &ShipResource, amount: f64) {
        self.0.insert(*resource, self.get(resource) + amount);
    }

    pub fn has(&self, resource: &ShipResource, amount: f64) -> bool {
        self.get(resource) >= amount
    }

    pub fn remove(&mut self, resource: &ShipResource, amount: f64) -> bool {
        if !self.has(resource, amount) {
            return false;
        }
        self.0.insert(*resource, self.get(resource) - amount);
        true
    }

    pub fn remove_available(&mut self, resource: &ShipResource, amount: f64) -> f64 {
        let available = self.get(resource);
        if available >= amount {
            self.0.insert(*resource, available - amount);
            amount
        } else {
            self.0.remove(resource).unwrap_or(0.0)
        }
    }

    pub fn min_of(&self, resources: &[ShipResource]) -> f64 {
        resources
            .iter()
            .map(|r| self.get(r))
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0)
    }

    pub fn max_of(&self, resources: &[ShipResource]) -> f64 {
        resources
            .iter()
            .map(|r| self.get(r))
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0)
    }
}
