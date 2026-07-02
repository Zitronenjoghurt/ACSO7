use crate::world::ship::resources::flow::{FlowMeter, FlowSource};
use std::collections::HashMap;
use strum::{EnumIter, IntoEnumIterator};

pub mod flow;
pub mod history;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, EnumIter,
)]
pub enum ShipResource {
    Power,
    Heat,
    Neutrons,
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
            ShipResource::Neutrons => "NEUT",
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

    pub fn base_capacity(&self) -> Option<f64> {
        match self {
            ShipResource::Power | ShipResource::Heat | ShipResource::Neutrons => None,
            ShipResource::Protium
            | ShipResource::Deuterium
            | ShipResource::Tritium
            | ShipResource::Helium3
            | ShipResource::Helium4
            | ShipResource::Lithium
            | ShipResource::Hydrogen => Some(50_000.0),
            ShipResource::Oxygen
            | ShipResource::Carbon
            | ShipResource::Nitrogen
            | ShipResource::CarbonDioxide
            | ShipResource::Methane => Some(20_000.0),
            ShipResource::Water => Some(100_000.0),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ShipResource::Power => "POWER",
            ShipResource::Heat => "HEAT",
            ShipResource::Neutrons => "NEUTRONS",
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

fn default_capacities() -> HashMap<ShipResource, f64> {
    ShipResource::iter()
        .filter_map(|resource| resource.base_capacity().map(|cap| (resource, cap)))
        .collect()
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ShipResources {
    amounts: HashMap<ShipResource, f64>,
    #[serde(default = "default_capacities")]
    capacities: HashMap<ShipResource, f64>,
    #[serde(skip)]
    meter: FlowMeter,
}

impl Default for ShipResources {
    fn default() -> Self {
        Self {
            amounts: HashMap::new(),
            capacities: default_capacities(),
            meter: FlowMeter::default(),
        }
    }
}

impl ShipResources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn capacity(&self, resource: &ShipResource) -> Option<f64> {
        self.capacities.get(resource).copied()
    }

    pub fn set_capacity(&mut self, resource: ShipResource, capacity: f64) {
        self.capacities.insert(resource, capacity);
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

    pub fn min_of(&self, resources: impl IntoIterator<Item = ShipResource>) -> f64 {
        resources
            .into_iter()
            .map(|r| self.get(&r))
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0)
    }

    pub fn max_of(&self, resources: impl IntoIterator<Item = ShipResource>) -> f64 {
        resources
            .into_iter()
            .map(|r| self.get(&r))
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
        let new_amount = self.get(resource) + amount;
        match self.capacity(resource) {
            Some(cap) if new_amount > cap => {
                self.amounts.insert(*resource, cap);
                self.meter
                    .record_consumed(FlowSource::Vented, *resource, new_amount - cap);
            }
            _ => {
                self.amounts.insert(*resource, new_amount);
            }
        }
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
