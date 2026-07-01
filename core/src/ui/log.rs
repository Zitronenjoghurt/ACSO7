use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct EventLog {
    entries: VecDeque<String>,
}

impl EventLog {
    const CAP: usize = 100;

    pub fn push(&mut self, message: impl Into<String>) {
        self.entries.push_back(message.into());
        while self.entries.len() > Self::CAP {
            self.entries.pop_front();
        }
    }

    pub fn recent(&self, n: usize) -> impl Iterator<Item = &String> {
        let skip = self.entries.len().saturating_sub(n);
        self.entries.iter().skip(skip)
    }
}
