use crate::world::ship::resources::flow::{FlowMeter, FlowSource};
use crate::world::ship::resources::{ShipResource, ShipResources};
use std::collections::HashMap;
use std::collections::VecDeque;

pub type Points = Vec<(f64, f64)>;

type Stock = HashMap<ShipResource, f64>;

pub const MIN_RATE: f64 = 0.05;

const TIERS: [(f64, usize, &str); 8] = [
    (0.1, 100, "10 S"),
    (1.0, 120, "2 MIN"),
    (60.0, 120, "2 HR"),
    (3600.0, 72, "3 DAY"),
    (86400.0, 90, "3 MON"),
    (2_592_000.0, 120, "10 YR"),
    (25_920_000.0, 120, "100 YR"),
    (259_200_000.0, 120, "1000 YR"),
];

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Sample {
    pub stock: Stock,
    pub flows: FlowMeter,
}

#[derive(Debug, Copy, Clone)]
pub struct SourceFlow {
    pub source: FlowSource,
    pub produced: f64,
    pub consumed: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Tier {
    interval: f64,
    cap: usize,
    elapsed: f64,
    #[serde(skip)]
    accum: FlowMeter,
    samples: VecDeque<Sample>,
}

impl Tier {
    fn new(interval: f64, cap: usize) -> Self {
        Self {
            interval,
            cap,
            elapsed: 0.0,
            accum: FlowMeter::default(),
            samples: VecDeque::new(),
        }
    }

    fn accumulate(&mut self, dt: f64, flows: &FlowMeter, stock: &Stock) {
        self.accum.merge(flows);
        self.elapsed += dt;
        if self.elapsed < self.interval {
            return;
        }
        let crossed = (self.elapsed / self.interval).floor();
        self.elapsed -= crossed * self.interval;
        let crossed = crossed as usize;
        let commits = crossed.min(self.cap);

        let prev = self
            .samples
            .back()
            .map(|s| s.stock.clone())
            .unwrap_or_else(|| stock.clone());
        let per_bucket = self.accum.scaled(1.0 / crossed as f64);
        for k in 0..commits {
            let frac = (k + 1) as f64 / commits as f64;
            self.samples.push_back(Sample {
                stock: lerp_stock(&prev, stock, frac),
                flows: per_bucket.clone(),
            });
        }
        self.accum = FlowMeter::default();
        while self.samples.len() > self.cap {
            self.samples.pop_front();
        }
    }
}

fn lerp_stock(from: &Stock, to: &Stock, t: f64) -> Stock {
    to.iter()
        .map(|(res, &b)| {
            let a = from.get(res).copied().unwrap_or(b);
            (*res, a + (b - a) * t)
        })
        .collect()
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceHistory {
    tiers: Vec<Tier>,
}

impl Default for ResourceHistory {
    fn default() -> Self {
        Self {
            tiers: TIERS
                .iter()
                .map(|(interval, cap, _)| Tier::new(*interval, *cap))
                .collect(),
        }
    }
}

impl ResourceHistory {
    pub const TIER_COUNT: usize = TIERS.len();

    pub fn tier_label(tier: usize) -> &'static str {
        TIERS[tier.min(TIERS.len() - 1)].2
    }

    pub fn advance(&mut self, dt: f64, res: &mut ShipResources) {
        if dt <= 0.0 {
            return;
        }
        let flows = res.take_meter();
        let stock = res.snapshot();
        for tier in &mut self.tiers {
            tier.accumulate(dt, &flows, &stock);
        }
    }

    fn tier(&self, tier: usize) -> &Tier {
        &self.tiers[tier.min(self.tiers.len() - 1)]
    }

    pub fn net_rate(&self, resource: ShipResource) -> f64 {
        let tier = &self.tiers[0];
        let Some(sample) = tier.samples.back() else {
            return 0.0;
        };
        let net: f64 = sample
            .flows
            .by_resource(resource)
            .map(|(_, flow)| flow.produced - flow.consumed)
            .sum();
        net / tier.interval
    }

    pub fn span_secs(&self, tier: usize) -> f64 {
        let tier = self.tier(tier);
        tier.samples.len() as f64 * tier.interval
    }

    pub fn stock_series(&self, tier: usize, resource: ShipResource) -> Points {
        self.tier(tier)
            .samples
            .iter()
            .enumerate()
            .map(|(i, sample)| {
                (
                    i as f64,
                    sample.stock.get(&resource).copied().unwrap_or(0.0),
                )
            })
            .collect()
    }

    pub fn flow_series(&self, tier: usize, resource: ShipResource) -> (Points, Points) {
        let tier = self.tier(tier);
        let mut produced = Vec::with_capacity(tier.samples.len());
        let mut consumed = Vec::with_capacity(tier.samples.len());
        for (i, sample) in tier.samples.iter().enumerate() {
            let mut p = 0.0;
            let mut c = 0.0;
            for (_, flow) in sample.flows.by_resource(resource) {
                p += flow.produced;
                c += flow.consumed;
            }
            produced.push((i as f64, p / tier.interval));
            consumed.push((i as f64, c / tier.interval));
        }
        (produced, consumed)
    }

    pub fn sources_of(&self, tier: usize, resource: ShipResource) -> Vec<SourceFlow> {
        let tier = self.tier(tier);
        let Some(sample) = tier.samples.back() else {
            return Vec::new();
        };
        let mut rows: Vec<SourceFlow> = sample
            .flows
            .by_resource(resource)
            .map(|(source, flow)| SourceFlow {
                source,
                produced: flow.produced / tier.interval,
                consumed: flow.consumed / tier.interval,
            })
            .filter(|s| s.produced >= MIN_RATE || s.consumed >= MIN_RATE)
            .collect();
        rows.sort_by(|a, b| (b.produced + b.consumed).total_cmp(&(a.produced + a.consumed)));
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::ship::resources::flow::FlowSource;

    #[test]
    fn finest_tier_commits_ten_samples_per_second() {
        let mut history = ResourceHistory::default();
        let mut res = ShipResources::new();

        res.produce(FlowSource::Reactor, &ShipResource::Power, 50.0);
        history.advance(0.05, &mut res);
        assert_eq!(history.tier(0).samples.len(), 0);

        history.advance(0.06, &mut res);
        assert_eq!(history.tier(0).samples.len(), 1);
    }

    #[test]
    fn per_second_tier_commits_one_sample_per_second() {
        let mut history = ResourceHistory::default();
        let mut res = ShipResources::new();

        res.produce(FlowSource::Reactor, &ShipResource::Power, 50.0);
        history.advance(0.5, &mut res);
        assert_eq!(history.tier(1).samples.len(), 0);

        history.advance(0.6, &mut res);
        assert_eq!(history.tier(1).samples.len(), 1);
    }

    #[test]
    fn tiers_capture_different_resolutions() {
        let mut history = ResourceHistory::default();
        let mut res = ShipResources::new();
        for _ in 0..120 {
            res.produce(FlowSource::Reactor, &ShipResource::Power, 10.0);
            history.advance(1.0, &mut res);
        }
        assert_eq!(history.tier(1).samples.len(), 120);
        assert_eq!(history.tier(2).samples.len(), 2);
    }

    #[test]
    fn big_time_jump_is_bounded_by_capacity() {
        let mut history = ResourceHistory::default();
        let mut res = ShipResources::new();
        history.advance(3_153_600_000.0, &mut res);
        for tier in &history.tiers {
            assert!(tier.samples.len() <= tier.cap);
        }
    }

    #[test]
    fn distributes_flow_across_catch_up_buckets() {
        let mut history = ResourceHistory::default();
        let mut res = ShipResources::new();
        res.produce(FlowSource::Reactor, &ShipResource::Power, 100.0);
        history.advance(5.0, &mut res);

        let (produced, _) = history.flow_series(1, ShipResource::Power);
        assert_eq!(produced.len(), 5);
        for (_, rate) in &produced {
            assert!((rate - 20.0).abs() < 1e-9, "rate {rate}");
        }
    }

    #[test]
    fn hides_negligible_flow_sources() {
        let mut history = ResourceHistory::default();
        let mut res = ShipResources::new();
        res.produce(FlowSource::Reactor, &ShipResource::Heat, 0.01);
        history.advance(1.0, &mut res);
        assert!(history.sources_of(1, ShipResource::Heat).is_empty());
    }

    #[test]
    fn attributes_flows_to_sources() {
        let mut history = ResourceHistory::default();
        let mut res = ShipResources::new();

        res.produce(FlowSource::Reactor, &ShipResource::Power, 100.0);
        res.set(ShipResource::Power, 100.0);
        res.consume_available(FlowSource::LifeSupport, &ShipResource::Power, 40.0);
        history.advance(1.0, &mut res);

        let sources = history.sources_of(1, ShipResource::Power);
        assert_eq!(sources.len(), 2);
        assert_eq!(
            sources
                .iter()
                .find(|s| s.source == FlowSource::Reactor)
                .unwrap()
                .produced,
            100.0
        );
        assert_eq!(
            sources
                .iter()
                .find(|s| s.source == FlowSource::LifeSupport)
                .unwrap()
                .consumed,
            40.0
        );
    }
}
