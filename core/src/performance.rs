use crate::math::ema::EMA;

#[derive(Debug, Default, Clone, Copy)]
pub struct Performance {
    pub update: PerformanceTimer,
    pub render: PerformanceTimer,
}

#[derive(Debug, Clone, Copy)]
pub struct PerformanceTimer {
    start: jiff::Timestamp,
    last_stop: Option<jiff::Timestamp>,
    ema_secs: EMA<f32>,
    ema_interval: EMA<f32>,
}

impl Default for PerformanceTimer {
    fn default() -> Self {
        Self {
            start: jiff::Timestamp::now(),
            last_stop: None,
            ema_secs: Default::default(),
            ema_interval: Default::default(),
        }
    }
}

impl PerformanceTimer {
    pub fn start(&mut self) {
        self.start = jiff::Timestamp::now();
    }

    pub fn stop(&mut self) {
        let now = jiff::Timestamp::now();
        let elapsed = now.duration_since(self.start).as_secs_f32().max(0.0);
        self.ema_secs.update(elapsed, 1200);

        if let Some(last) = self.last_stop {
            let interval = now.duration_since(last).as_secs_f32();
            if interval > 0.0 {
                self.ema_interval.update(interval, 1200);
            }
        }
        self.last_stop = Some(now);
    }

    pub fn average_secs(&self) -> f32 {
        self.ema_secs.value()
    }

    pub fn display_average_secs(&self) -> String {
        format!("{:.02}ms", self.average_secs() * 1000.0)
    }

    pub fn updates_per_sec(&self) -> f32 {
        let interval = self.ema_interval.value();
        if interval > 0.0 { 1.0 / interval } else { 0.0 }
    }

    pub fn display_updates_per_sec(&self) -> String {
        format!("{:.01}/s", self.updates_per_sec())
    }

    pub fn budget(&self) -> f32 {
        self.average_secs() * self.updates_per_sec()
    }

    pub fn display_budget(&self) -> String {
        format!("{:.01}%", self.budget() * 100.0)
    }
}
