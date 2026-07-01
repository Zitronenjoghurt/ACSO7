use crate::world::ship::resources::ShipResource;
use std::collections::HashMap;
use strum::EnumIter;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, EnumIter,
)]
pub enum FlowSource {
    Reactor,
    LifeSupport,
}

impl FlowSource {
    pub fn label(&self) -> &'static str {
        match self {
            FlowSource::Reactor => "REACTOR",
            FlowSource::LifeSupport => "LIFE SUPPORT",
        }
    }
}

#[derive(Debug, Default, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Flow {
    pub produced: f64,
    pub consumed: f64,
}

impl Flow {
    pub fn net(&self) -> f64 {
        self.produced - self.consumed
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlowMeter(HashMap<FlowSource, HashMap<ShipResource, Flow>>);

impl FlowMeter {
    pub fn record_produced(&mut self, source: FlowSource, resource: ShipResource, amount: f64) {
        self.0
            .entry(source)
            .or_default()
            .entry(resource)
            .or_default()
            .produced += amount;
    }

    pub fn record_consumed(&mut self, source: FlowSource, resource: ShipResource, amount: f64) {
        self.0
            .entry(source)
            .or_default()
            .entry(resource)
            .or_default()
            .consumed += amount;
    }

    pub fn scaled(&self, factor: f64) -> FlowMeter {
        let mut out = FlowMeter::default();
        for (source, flows) in &self.0 {
            let entry = out.0.entry(*source).or_default();
            for (resource, flow) in flows {
                entry.insert(
                    *resource,
                    Flow {
                        produced: flow.produced * factor,
                        consumed: flow.consumed * factor,
                    },
                );
            }
        }
        out
    }

    pub fn merge(&mut self, other: &FlowMeter) {
        for (source, flows) in &other.0 {
            let entry = self.0.entry(*source).or_default();
            for (resource, flow) in flows {
                let slot = entry.entry(*resource).or_default();
                slot.produced += flow.produced;
                slot.consumed += flow.consumed;
            }
        }
    }

    pub fn by_resource(
        &self,
        resource: ShipResource,
    ) -> impl Iterator<Item = (FlowSource, Flow)> + '_ {
        self.0
            .iter()
            .filter_map(move |(source, flows)| flows.get(&resource).map(|flow| (*source, *flow)))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
