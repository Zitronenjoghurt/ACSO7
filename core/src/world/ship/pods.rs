use crate::world::colonist::Colonist;
use crate::world::events::WorldEvents;
use crate::world::ship::alert::Alert;

const MAX_DMG_PER_SEC: f64 = 0.01;
const INTEGRITY_WARNING: f64 = 0.5;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pods {
    pub pods: Vec<Pod>,
    pub pod_power_demand: f64,
    pub power_saturation: f64,
}

impl Pods {
    pub const MIN_SAFE_SATURATION: f64 = 0.99;

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

    pub fn life_support_failing(&self) -> bool {
        !self.pods.is_empty() && self.power_saturation < Self::MIN_SAFE_SATURATION
    }

    pub fn alerts(&self) -> Vec<Alert> {
        if self.pods.is_empty() {
            return vec![Alert::critical("ALL PODS DESTROYED")];
        }

        let mut alerts = Vec::new();
        if self.life_support_failing() {
            alerts.push(Alert::critical("LIFE SUPPORT LOSS"));
        }
        if !self.pods.is_empty() && self.avg_health() < INTEGRITY_WARNING {
            alerts.push(Alert::warning("POD INTEGRITY LOW"));
        }
        alerts
    }

    pub fn supply_power(&mut self, dt: f64, power: f64) {
        let demand = self.power_demand(dt);
        self.power_saturation = if demand > 0.0 {
            (power / demand).min(1.0)
        } else {
            1.0
        };
    }

    pub fn tick(&mut self, dt: f64, events: &mut WorldEvents) {
        if self.power_saturation >= Self::MIN_SAFE_SATURATION {
            return;
        }

        let unsaturation = 1.0 - self.power_saturation;
        let unsaturated_count = (unsaturation * self.pods.len() as f64).ceil() as usize;
        for _ in 0..unsaturated_count {
            let pod_index = fastrand::usize(..self.pods.len());
            let pod = &mut self.pods[pod_index];
            let dmg = fastrand::f64() * MAX_DMG_PER_SEC * unsaturation * dt;
            pod.health -= dmg;
            if pod.health <= 0.0 {
                events.pod_destroyed(pod.colonist);
                self.pods.swap_remove(pod_index);
            }
        }
    }
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
