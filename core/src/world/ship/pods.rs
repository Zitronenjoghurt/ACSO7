use crate::world::colonist::Colonist;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pods {
    pub pods: Vec<Pod>,
    pub pod_power_demand: f64,
    pub power_saturation: f64,
}

impl Pods {
    pub fn generate(count: usize, rng: &mut fastrand::Rng) -> Self {
        Self {
            pods: (0..count).map(|_| Pod::generate(rng)).collect(),
            pod_power_demand: 0.1,
            power_saturation: 0.0,
        }
    }

    pub fn power_demand(&self, dt: f64) -> f64 {
        self.pod_power_demand * self.pods.len() as f64 * dt
    }

    pub fn avg_health(&self) -> f64 {
        if self.pods.is_empty() {
            return 0.0;
        }
        self.pods.iter().map(|p| p.health).sum::<f64>() / self.pods.len() as f64
    }

    pub fn supply_power(&mut self, dt: f64, power: f64) {
        let demand = self.power_demand(dt);
        self.power_saturation = if demand > 0.0 {
            (power / demand).min(1.0)
        } else {
            1.0
        };
    }

    pub fn tick(&mut self, _dt: f64) {}
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pod {
    pub colonist: Colonist,
    pub health: f64,
}

impl Pod {
    pub fn generate(rng: &mut fastrand::Rng) -> Self {
        Self {
            colonist: Colonist::generate(rng),
            health: 1.0,
        }
    }
}
