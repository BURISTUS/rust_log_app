use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStats {
    pub total_clients: u64,
    pub total_clients_unserviced: u64,
    pub total_duration: Duration,
    pub max_duration: Option<Duration>,
    pub min_duration: Option<Duration>,
}

impl ServerStats {
    pub fn new() -> Self {
        ServerStats {
            total_clients: 0,
            total_clients_unserviced: 0,
            total_duration: Duration::from_secs(0),
            max_duration: None,
            min_duration: None,
        }
    }

    pub fn average_duration(&self) -> Option<Duration> {
        if self.total_clients > 0 {
            Some(Duration::from_secs_f64(
                self.total_duration.as_secs_f64() / self.total_clients as f64,
            ))
        } else {
            None
        }
    }

    pub fn total_unserviced_clients(&self) -> u64 {
        self.total_clients_unserviced
    }
}

impl Default for ServerStats {
    fn default() -> Self {
        Self::new()
    }
}
