use std::{collections::HashMap, time::Instant};

#[derive(Debug, Default)]
pub struct Registry {
    pub devices: HashMap<String, Device>,
}

impl Registry {
    pub fn needs_pruning(&self, expiry: f32) -> bool {
        self.devices.values().any(|device| {
            let elapsed = Instant::now()
                .duration_since(device.last_updated)
                .as_secs_f32();
            elapsed >= expiry
        })
    }

    pub fn prune(&mut self, expiry: f32) {
        let now = Instant::now();

        self.devices.retain(|_, device| {
            let elapsed = now.duration_since(device.last_updated).as_secs_f32();
            elapsed < expiry
        });
    }
}

#[derive(Debug)]
pub struct Device {
    stats: HashMap<&'static str, f32>,
    last_updated: Instant,
}

impl Device {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
            last_updated: Instant::now(),
        }
    }

    pub fn stats(&self) -> &HashMap<&'static str, f32> {
        &self.stats
    }

    pub fn update(&mut self, name: &'static str, value: f32) {
        self.stats.insert(name, value);
        self.last_updated = Instant::now();
    }
}
