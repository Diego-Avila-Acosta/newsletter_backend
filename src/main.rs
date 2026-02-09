use std::fmt::{Debug, Display};

use newsletter_backend::configuration::get_configuration;
use newsletter_backend::issue_delivery_worker::run_worker_until_stopped;
use newsletter_backend::metrics::init_prometheus_exporter;
use newsletter_backend::startup::Application;
use newsletter_backend::telemetry::{get_opentelemetry_parts, get_subscriber, init_subscriber};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Error reading initial configuration");

    let (tracer, provider) = get_opentelemetry_parts(
        &configuration.tracer.export_endpoint,
        configuration.tracer.sampling_ratio,
    );

    let subscriber = get_subscriber(tracer, "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    init_prometheus_exporter();

    let configuration = get_configuration().expect("Error reading initial configuration");

    let server = Application::build(configuration.clone()).await?;
    let server_task = tokio::spawn(server.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(configuration));

    tokio::select! {
        o = server_task => report_exit("API", o),
        o = worker_task => report_exit("Background worker", o),
    };

    provider
        .shutdown()
        .expect("Failed to close tracer provider");

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(_)) => {
            tracing::info!("{} has exited", task_name)
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
            "{} failed failed to complete",
            task_name
            )
        }
    }
}
