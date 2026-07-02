use crate::world::events::WorldEvents;
use crate::world::ship::resources::ShipResource;
use crate::world::ship::resources::flow::FlowSource;
use crate::world::ship::resources::history::ResourceHistory;

pub mod alert;
pub mod pods;
pub mod reactor;
pub mod resources;

const TRITIUM_HALF_LIFE_SECS: f64 = 388_158_480.0;

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
    pub fn tick(&mut self, dt: f64, events: &mut WorldEvents) {
        self.tritium_decay(dt);
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

    fn tritium_decay(&mut self, dt: f64) {
        let tritium = self.res.get(&ShipResource::Tritium);
        if tritium <= 0.0 {
            return;
        }
        let fraction = 1.0 - 0.5_f64.powf(dt / TRITIUM_HALF_LIFE_SECS);
        let max_decay = tritium * fraction;
        let decayed =
            self.res
                .consume_available(FlowSource::TritiumDecay, &ShipResource::Tritium, max_decay);
        self.res
            .produce(FlowSource::TritiumDecay, &ShipResource::Helium3, decayed);
    }
}
