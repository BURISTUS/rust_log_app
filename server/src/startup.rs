use crate::{
    middleware::BeforeRequest,
    routes::health_check,
    stats::{ClientStats, ServerStats},
    utils::conifg::{ApplicationSettings, ConnectionConfig},
};
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

pub struct Application {
    port: u16,
    server: Server,
}

#[derive(Debug)]
pub struct AppState {
    pub server_stats: Arc<Mutex<ServerStats>>,
    pub client_stats: Arc<Mutex<HashMap<String, ClientStats>>>,
    pub sem: Arc<tokio::sync::Semaphore>,
}

impl Application {
    pub async fn build(app_configuration: ApplicationSettings) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            app_configuration.application.host, app_configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, app_configuration.connection)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stoped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(
    listener: TcpListener,
    connection_config: ConnectionConfig,
) -> Result<Server, anyhow::Error> {
    let sem = Arc::new(tokio::sync::Semaphore::new(5));
    let client_stats = Arc::new(Mutex::new(HashMap::new()));
    let server_stats = Arc::new(Mutex::new(ServerStats::new()));
    let app_state = web::Data::new(AppState {
        server_stats,
        client_stats,
        sem,
    });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(BeforeRequest)
            .route("/health_check", web::get().to(health_check))
            // .wrap(AfterRequest)
            .app_data(connection_config.clone())
            .app_data(app_state.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
