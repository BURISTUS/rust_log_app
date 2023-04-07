use clap::Parser;
use reqwest::Client;
use std::time::{Duration, Instant};
use tokio::task;
use client::{cmd::Args, utils::get_configuration};

async fn send_request(client: Client, url: String) -> Result<Duration, String> {
    let start_time = Instant::now();
    let response = client.get(&url).send().await;
    match response {
        Ok(response) => {
            let duration = Instant::now() - start_time;
            if response.status().is_success() {
                Ok(duration)
            } else {
                Err(format!("Error: status code {:?}", response.status()))
            }
        }
        Err(e) => Err(format!("Error: {:?}", e)),
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = get_configuration()?;
    if args.requests < 1 || args.requests > 100 {
        eprintln!("The number of requests must be between 1 and 100");
        return Ok(());
    }

    let server_url = "http://localhost:8000/health_check";
    let client = Client::builder()
        .timeout(Duration::from_secs(config.settings.timeout))
        .build()?;

    let requests = (0..args.requests).map(|_| {
    let url = server_url.to_string();
    let cloned_client = client.clone();
    task::spawn(send_request(cloned_client, url))
});

    let mut min_duration = Duration::MAX;
    let mut max_duration = Duration::ZERO;
    let mut total_duration = Duration::new(0, 0);
    let mut responses_received = 0;

    for req in requests {
        match req.await {
            Ok(Ok(duration)) => {
                min_duration = min_duration.min(duration);
                max_duration = max_duration.max(duration);
                total_duration += duration;
                responses_received += 1;
            }
            Ok(Err(e)) => eprintln!("Request error: {:?}", e),
            Err(e) => eprintln!("Task error: {:?}", e),
        }
    }

    println!("Total responses received: {}", responses_received);
    if responses_received > 0 {
        let avg_duration = total_duration / responses_received;
        println!("Min response time: {:?}", min_duration);
        println!("Max response time: {:?}", max_duration);
        println!("Average response time: {:?}", avg_duration);
    }
    println!("Total data exchange time: {:?}", total_duration);

    Ok(())
}
