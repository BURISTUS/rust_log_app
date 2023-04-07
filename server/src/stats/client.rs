use std::time::Duration;
use std::u32;

#[derive(Debug)]
pub struct ClientStats {
    pub request_count: u32,
    pub max_duration: Option<Duration>,
    pub min_duration: Option<Duration>,
    pub total_duration: Duration,
}

impl ClientStats {
    pub fn new(min_time: u64, max_time: u64) -> Self {
        Self {
            request_count: 0,
            max_duration: Some(Duration::from_millis(min_time)),
            min_duration: Some(Duration::from_millis(max_time)),
            total_duration: std::time::Duration::default(),
        }
    }

    fn get_avg_duration(&self) -> Option<Duration> {
        if self.request_count > 0 {
            Some(self.total_duration / self.request_count)
        } else {
            None
        }
    }

    pub fn display_stats(&self) {
        log::info!("Request count: {}", self.request_count);
        log::info!("Max request duration: {:?}", self.max_duration);
        log::info!("Min request duration: {:?}", self.min_duration);
        log::info!("Total request duration: {:?}", self.total_duration);
        log::info!("Average request duration: {:?}", self.get_avg_duration());
    }
}
