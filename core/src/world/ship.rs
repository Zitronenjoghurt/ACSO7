use crate::world::ship::resources::ShipResource;

pub mod pods;
mod power_router;
mod reactor;
pub mod resources;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Ship {
    pub res: resources::ShipResources,
    pub reactor: reactor::Reactor,
    pub pods: pods::Pods,
    pub power_router: power_router::PowerRouter,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            res: resources::ShipResources::default(),
            reactor: reactor::Reactor::default(),
            pods: pods::Pods::generate(1000, &mut fastrand::Rng::new()),
            power_router: power_router::PowerRouter::default(),
        }
    }
}

impl Ship {
    pub fn tick(&mut self, dt: f64) {
        self.reactor.tick(dt, &mut self.res);
        self.supply_power(dt);
        self.pods.tick(dt);
    }

    fn supply_power(&mut self, dt: f64) {
        let total_demand = self.pods.power_demand(dt);
        let power_to_supply = self
            .res
            .remove_available(&ShipResource::Power, total_demand);
        self.pods.supply_power(dt, power_to_supply);
    }
}
