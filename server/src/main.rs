use env_logger::Env;
use server::{
    startup::Application,
    stats::ServerStats,
    utils::{conifg::get_configuration, read_from_file},
};
use std::fmt::{Debug, Display};
use tokio::task::JoinError;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let configuration = get_configuration().expect("Failed to get configuration");
    println!("{:?}", configuration);
    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stoped());
    let server_stats = read_from_file()?;
    tokio::select! {
        o = application_task => report_exit("API", server_stats, o),
    };

    Ok(())
}

fn report_exit(
    task_name: &str,
    server_stats: ServerStats,
    outcome: Result<Result<(), impl Debug + Display>, JoinError>,
) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name);
            log::info!("Total duration: {:?}", server_stats.total_duration);
            log::info!("Min request duration {:?}", server_stats.min_duration);
            log::info!("Max request duration {:?}", server_stats.max_duration);
            log::info!("Total clients {:?}", server_stats.total_clients);
            log::info!(
                "Total unserviced clients {:?}",
                server_stats.total_clients_unserviced
            );
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
// fn report_exit(task_name: &str, outcome: Result<>)
