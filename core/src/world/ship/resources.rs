use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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
