use std::sync::LazyLock;

use opentelemetry::KeyValue;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::Sampler;
use opentelemetry_sdk::trace::{SdkTracer, SdkTracerProvider};
use opentelemetry_semantic_conventions::resource;
use tokio::task::JoinHandle;
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, fmt::MakeWriter, layer::SubscriberExt};

const APP_NAME: &str = "newsletter_backend";

static RESOURCE: LazyLock<Resource> = LazyLock::new(|| {
    Resource::builder()
        .with_attribute(KeyValue::new(resource::SERVICE_NAME, APP_NAME))
        .build()
});

pub fn get_subscriber<Sink>(
    tracer: SdkTracer,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let formatting_layer = BunyanFormattingLayer::new(APP_NAME.into(), sink);

    Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber")
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}

pub fn get_opentelemetry_parts(
    export_endpoint: &str,
    sampling_ratio: f64,
) -> (SdkTracer, SdkTracerProvider) {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(export_endpoint)
        .build()
        .expect("Failed to build span exporter");

    let sampler = Sampler::TraceIdRatioBased(sampling_ratio);

    let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_sampler(sampler)
        .with_batch_exporter(otlp_exporter)
        .with_resource(RESOURCE.clone())
        .build();

    let tracer = provider.tracer(APP_NAME);

    (tracer, provider)
}
