use crate::world::ship::alert::Alert;
use crate::world::ship::resources::flow::FlowSource;
use crate::world::ship::resources::{ShipResource, ShipResources};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum ReactorMode {
    /// D + He-3 -> He-4 + p. Aneutronic: all energy in charged particles, ~no waste heat.
    /// Burns expensive He-3 fuel. Hottest mode (~1.16e9 K).
    #[default]
    DHe3,
    /// D + D -> T + p (~50%) or He-3 + n (~50%). Run cool (~4.6e8 K) so T/He-3 survive to be harvested.
    /// Low energy yield, but breeds sellable T + He-3. ~1/4 of output is neutrons.
    DD,
    /// Catalyzed DD: T and He-3 stay in the plasma and re-fuse with D. Net: 6 D -> 2 He-4 + 2 p + 2 n.
    /// Max energy from cheap D, but heaviest neutron load (incl. 14.1 MeV D-T neutron) -> most cooling.
    CatDD,
}

impl ReactorMode {
    pub const ALL: [ReactorMode; 3] = [ReactorMode::DHe3, ReactorMode::DD, ReactorMode::CatDD];

    pub fn label(&self) -> &'static str {
        match self {
            ReactorMode::DHe3 => "D-HE³",
            ReactorMode::DD => "D-D",
            ReactorMode::CatDD => "CAT-DD",
        }
    }

    pub fn next(self) -> Self {
        match self {
            ReactorMode::DHe3 => ReactorMode::DD,
            ReactorMode::DD => ReactorMode::CatDD,
            ReactorMode::CatDD => ReactorMode::DHe3,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            ReactorMode::DHe3 => ReactorMode::CatDD,
            ReactorMode::DD => ReactorMode::DHe3,
            ReactorMode::CatDD => ReactorMode::DD,
        }
    }

    pub fn help_lines(&self) -> &'static [&'static str] {
        match self {
            ReactorMode::DHe3 => &[
                "IN   Deuterium + Helium-3",
                "OUT  Helium-4 + Protium",
                "Clean aneutronic burn, near-zero heat.",
                "Top power, but He-3 is scarce.",
            ],
            ReactorMode::DD => &[
                "IN   Deuterium",
                "OUT  Tritium + Helium-3 + Protium + Neutrons",
                "Weak output, breeds useful T + He-3.",
            ],
            ReactorMode::CatDD => &[
                "IN   Deuterium",
                "OUT  Helium-4 + Protium + Neutrons",
                "Max power from cheap D, heavy neutron flux.",
            ],
        }
    }

    pub fn inputs(&self) -> &[(ShipResource, f64)] {
        match self {
            ReactorMode::DHe3 => &[(ShipResource::Deuterium, 0.5), (ShipResource::Helium3, 0.5)],
            ReactorMode::DD => &[(ShipResource::Deuterium, 1.0)],
            ReactorMode::CatDD => &[(ShipResource::Deuterium, 1.0)],
        }
    }

    pub fn outputs(&self) -> &[(ShipResource, f64)] {
        match self {
            ReactorMode::DHe3 => &[(ShipResource::Helium4, 1.0), (ShipResource::Protium, 1.0)],
            ReactorMode::DD => &[
                (ShipResource::Tritium, 0.25),
                (ShipResource::Helium3, 0.25),
                (ShipResource::Protium, 0.25),
                (ShipResource::Neutrons, 0.25),
            ],
            ReactorMode::CatDD => &[
                (ShipResource::Helium4, 1.0 / 3.0),
                (ShipResource::Protium, 1.0 / 3.0),
                (ShipResource::Neutrons, 1.0 / 3.0),
            ],
        }
    }

    pub fn efficiency(&self) -> f64 {
        match self {
            ReactorMode::DHe3 => 0.9995,
            ReactorMode::DD => 0.80,
            ReactorMode::CatDD => 0.75,
        }
    }

    /// Energy released per reaction.
    pub const fn energy_yield(&self) -> f64 {
        match self {
            ReactorMode::DHe3 => 9.15,
            ReactorMode::DD => 1.12,
            ReactorMode::CatDD => 4.45,
        }
    }

    pub const fn target_temp_kelvin(&self) -> f64 {
        match self {
            ReactorMode::DHe3 => 1.16e9,
            ReactorMode::DD => 4.6e8,
            ReactorMode::CatDD => 8.1e8,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Reactor {
    pub mode: ReactorMode,
    pub health: f64,
    pub fusion_rate: f64,
    pub fusion_saturation: f64,
}

impl Default for Reactor {
    fn default() -> Self {
        Self {
            mode: ReactorMode::default(),
            health: 1.0,
            fusion_rate: 100.0,
            fusion_saturation: 0.0,
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
        if self.fusion_saturation < 0.1 {
            alerts.push(Alert::warning("LACKING FUSION FUEL"));
        }
        alerts
    }

    pub fn tick(&mut self, dt: f64, res: &mut ShipResources) {
        let max_rate = self
            .mode
            .inputs()
            .iter()
            .map(|(resource, factor)| res.get(resource) / factor)
            .fold(f64::INFINITY, f64::min);
        let base_rate = self.fusion_rate.min(max_rate) * self.health;
        let rate = base_rate * dt;
        self.fusion_saturation = base_rate / self.fusion_rate;
        if rate == 0.0 {
            return;
        }

        for (input, factor) in self.mode.inputs() {
            res.consume(FlowSource::Reactor, input, rate * factor);
        }

        for (output, factor) in self.mode.outputs() {
            res.produce(FlowSource::Reactor, output, rate * factor);
        }

        let base_yield = self.mode.energy_yield() * rate;
        let energy_yield = base_yield * self.mode.efficiency();
        res.produce(FlowSource::Reactor, &ShipResource::Power, energy_yield);

        let heat_yield = base_yield * (1.0 - self.mode.efficiency());
        res.produce(FlowSource::Reactor, &ShipResource::Heat, heat_yield);
    }
}
