//! This crate is using the previous version of `opentelemetry-*` and `tracing-opentelemetry` crates.
//! It's working fine.

use opentelemetry::{global::shutdown_tracer_provider, KeyValue};
use opentelemetry_sdk::{runtime, Resource};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tracing::instrument]
fn foo() {
    tracing::info!("info");
    tracing::error!("error");
    bar();
}

#[tracing::instrument]
fn bar() {
    tracing::info!("info");
    tracing::error!("error");
}

#[tokio::main]
async fn main() {
    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
            Resource::new(vec![KeyValue::new("service.name", "issue-01")]),
        ))
        .install_batch(runtime::Tokio)
        .unwrap();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(provider))
        .init();

    foo();

    shutdown_tracer_provider();
}
