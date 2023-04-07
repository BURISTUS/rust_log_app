use crate::stats::ClientStats;
use crate::{startup::AppState, utils::write_server_data_to_file};
use actix_web::{web, HttpResponse};
use rand::Rng;
use std::time::{Duration, Instant};

pub async fn health_check(app_state: web::Data<AppState>) -> HttpResponse {
    let _guard = app_state.sem.acquire().await.unwrap();

    let mut rng = rand::thread_rng();
    let processing_time = Duration::from_millis(rng.gen_range(100..=500));
    let start_time = Instant::now();
    tokio::time::sleep(processing_time).await;

    let response = "Request processed.".to_string();
    let elapsed_time = start_time.elapsed();
    let client_addr = "client_address";

    {
        let mut stats = app_state.server_stats.lock().unwrap();

        stats.total_clients += 1;
        stats.total_duration += elapsed_time;
        stats.max_duration = stats
            .max_duration
            .map(|d| d.max(elapsed_time))
            .or(Some(elapsed_time));
        stats.min_duration = stats
            .min_duration
            .map(|d| d.min(elapsed_time))
            .or(Some(elapsed_time));
        stats.total_clients_unserviced -= 1;

        write_server_data_to_file(&stats).expect("Can't write to file");
    }

    let mut client_stats_map = app_state.client_stats.lock().unwrap();

    let client_stats = client_stats_map
        .entry(client_addr.to_string())
        .or_insert_with(|| ClientStats::new(100, 500));

    client_stats.request_count += 1;
    client_stats.total_duration += elapsed_time;
    client_stats.max_duration = client_stats
        .max_duration
        .map(|d| d.max(elapsed_time))
        .or(Some(elapsed_time));
    client_stats.min_duration = client_stats
        .min_duration
        .map(|d| d.min(elapsed_time))
        .or(Some(elapsed_time));

    client_stats.display_stats();
    println!(
        "Client {}: {} requests processed",
        client_addr, client_stats.request_count
    );

    HttpResponse::Ok().body(response)
}
