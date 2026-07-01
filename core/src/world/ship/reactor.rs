use crate::world::ship::alert::Alert;
use crate::world::ship::resources::flow::FlowSource;
use crate::world::ship::resources::{ShipResource, ShipResources};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum ReactorMode {
    #[default]
    /// D + He-3 -> He-4 + p
    DHe3,
}

impl ReactorMode {
    pub fn rate(&self) -> f64 {
        match self {
            ReactorMode::DHe3 => 100.0,
        }
    }

    pub fn inputs(&self) -> &[ShipResource] {
        match self {
            ReactorMode::DHe3 => &[ShipResource::Deuterium, ShipResource::Helium3],
        }
    }

    pub fn outputs(&self) -> &[ShipResource] {
        match self {
            ReactorMode::DHe3 => &[ShipResource::Helium4, ShipResource::Protium],
        }
    }

    pub fn efficiency(&self) -> f64 {
        match self {
            ReactorMode::DHe3 => 0.9995,
        }
    }

    /// Energy released per reaction.
    pub const fn energy_yield(&self) -> f64 {
        match self {
            ReactorMode::DHe3 => 18.5,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Reactor {
    pub mode: ReactorMode,
    pub health: f64,
}

impl Default for Reactor {
    fn default() -> Self {
        Self {
            mode: ReactorMode::default(),
            health: 1.0,
        }
    }
}

impl Reactor {
    pub fn alerts(&self) -> Vec<Alert> {
        let mut alerts = Vec::new();
        if self.health <= 0.0 {
            alerts.push(Alert::critical("REACTOR OFFLINE"));
        } else if self.health < 1.0 {
            alerts.push(Alert::warning("REACTOR DAMAGED"));
        }
        alerts
    }

    pub fn tick(&mut self, dt: f64, res: &mut ShipResources) {
        let min_res = res.min_of(self.mode.inputs());
        let rate = self.mode.rate().min(min_res) * self.health * dt;
        if rate == 0.0 {
            return;
        }

        for input in self.mode.inputs() {
            res.consume(FlowSource::Reactor, input, rate);
        }

        for output in self.mode.outputs() {
            res.produce(FlowSource::Reactor, output, rate);
        }

        let base_yield = self.mode.energy_yield() * rate;
        let energy_yield = base_yield * self.mode.efficiency();
        res.produce(FlowSource::Reactor, &ShipResource::Power, energy_yield);

        let heat_yield = base_yield * (1.0 - self.mode.efficiency());
        res.produce(FlowSource::Reactor, &ShipResource::Heat, heat_yield);
    }
}
