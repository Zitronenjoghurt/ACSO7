use crate::world::ship::resources::flow::{FlowMeter, FlowSource};
use std::collections::HashMap;
use strum::EnumIter;

pub mod flow;
pub mod history;

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

    pub fn name(&self) -> &'static str {
        match self {
            ShipResource::Power => "POWER",
            ShipResource::Heat => "HEAT",
            ShipResource::Protium => "PROTIUM",
            ShipResource::Deuterium => "DEUTERIUM",
            ShipResource::Tritium => "TRITIUM",
            ShipResource::Helium3 => "HELIUM-3",
            ShipResource::Helium4 => "HELIUM-4",
            ShipResource::Lithium => "LITHIUM",
            ShipResource::Hydrogen => "HYDROGEN",
            ShipResource::Oxygen => "OXYGEN",
            ShipResource::Carbon => "CARBON",
            ShipResource::Nitrogen => "NITROGEN",
            ShipResource::Water => "WATER",
            ShipResource::CarbonDioxide => "CARBON DIOXIDE",
            ShipResource::Methane => "METHANE",
        }
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ShipResources {
    amounts: HashMap<ShipResource, f64>,
    #[serde(skip)]
    meter: FlowMeter,
}

impl ShipResources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, resource: &ShipResource) -> f64 {
        self.amounts.get(resource).copied().unwrap_or(0.0)
    }

    pub fn set(&mut self, resource: ShipResource, amount: f64) {
        self.amounts.insert(resource, amount);
    }

    pub fn has(&self, resource: &ShipResource, amount: f64) -> bool {
        self.get(resource) >= amount
    }

    pub fn produce(&mut self, source: FlowSource, resource: &ShipResource, amount: f64) {
        self.add(resource, amount);
        self.meter.record_produced(source, *resource, amount);
    }

    pub fn consume(&mut self, source: FlowSource, resource: &ShipResource, amount: f64) -> bool {
        if !self.remove(resource, amount) {
            return false;
        }
        self.meter.record_consumed(source, *resource, amount);
        true
    }

    pub fn consume_available(
        &mut self,
        source: FlowSource,
        resource: &ShipResource,
        amount: f64,
    ) -> f64 {
        let removed = self.remove_available(resource, amount);
        self.meter.record_consumed(source, *resource, removed);
        removed
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

    pub fn snapshot(&self) -> HashMap<ShipResource, f64> {
        self.amounts.clone()
    }

    pub fn take_meter(&mut self) -> FlowMeter {
        std::mem::take(&mut self.meter)
    }

    fn add(&mut self, resource: &ShipResource, amount: f64) {
        self.amounts.insert(*resource, self.get(resource) + amount);
    }

    fn remove(&mut self, resource: &ShipResource, amount: f64) -> bool {
        if !self.has(resource, amount) {
            return false;
        }
        self.amounts.insert(*resource, self.get(resource) - amount);
        true
    }

    fn remove_available(&mut self, resource: &ShipResource, amount: f64) -> f64 {
        let available = self.get(resource);
        if available >= amount {
            self.amounts.insert(*resource, available - amount);
            amount
        } else {
            self.amounts.remove(resource).unwrap_or(0.0)
        }
    }
}
