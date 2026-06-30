pub mod pods;
mod power_router;
mod reactor;
pub mod resources;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Ship {
    pub res: resources::ShipResources,
    pub reactor: reactor::Reactor,
    pub power_router: power_router::PowerRouter,
}

impl Ship {
    pub fn tick(&mut self, dt: f64) {
        self.reactor.tick(dt, &mut self.res);
    }
}
