use crate::world::events::WorldEvents;
use crate::world::ship::alert::Alert;
use crate::world::ship::pods::Pods;
use crate::world::ship::resources::ShipResource;
use crate::world::ship::resources::flow::FlowSource;
use crate::world::ship::resources::history::ResourceHistory;

pub mod alert;
pub mod pods;
mod reactor;
pub mod resources;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Ship {
    pub res: resources::ShipResources,
    pub reactor: reactor::Reactor,
    pub pods: pods::Pods,
    #[serde(default)]
    pub history: ResourceHistory,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            res: resources::ShipResources::default(),
            reactor: reactor::Reactor::default(),
            pods: pods::Pods::generate(1000, &mut fastrand::Rng::new()),
            history: ResourceHistory::default(),
        }
    }
}

impl Ship {
    pub fn grid_alerts(&self) -> Vec<Alert> {
        if self.pods.power_saturation < Pods::MIN_SAFE_SATURATION {
            vec![Alert::warning("GRID UNDERPOWERED")]
        } else {
            Vec::new()
        }
    }

    pub fn tick(&mut self, dt: f64, events: &mut WorldEvents) {
        self.reactor.tick(dt, &mut self.res);
        self.supply_power(dt);
        self.pods.tick(dt, events);
        self.history.advance(dt, &mut self.res);
    }

    fn supply_power(&mut self, dt: f64) {
        let total_demand = self.pods.power_demand(dt);
        let power_to_supply =
            self.res
                .consume_available(FlowSource::LifeSupport, &ShipResource::Power, total_demand);
        self.pods.supply_power(dt, power_to_supply);
    }
}
