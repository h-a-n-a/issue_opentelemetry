//! This demo is using the latest version of `opentelemetry-*` and `tracing-opentelemetry` crates.
//!
//! None of these tests are working.
//! `subscriber_init` exits without any error but not generating any trace.
//! `subscriber_with_default` and `subscriber_set_default` are stuck at `shutdown_tracer_provider` and not generating any trace.

use opentelemetry::{
    global::{self, shutdown_tracer_provider},
    trace::TracerProvider,
    KeyValue,
};
use opentelemetry_sdk::{runtime, Resource};
use tracing::subscriber::{set_default, with_default};
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

fn main() {}

#[tokio::test]
async fn subscriber_init() {
    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
            Resource::new(vec![KeyValue::new("service.name", "issue-02")]),
        ))
        .install_batch(runtime::Tokio)
        .unwrap();
    global::set_tracer_provider(provider.clone());

    let trace = provider.tracer("issue-02");

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(trace))
        .init();

    foo();

    shutdown_tracer_provider();
}

#[tokio::test]
async fn subscriber_with_default() {
    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
            Resource::new(vec![KeyValue::new("service.name", "issue-02")]),
        ))
        .install_batch(runtime::Tokio)
        .unwrap();
    global::set_tracer_provider(provider.clone());

    let trace = provider.tracer("issue-02");

    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(trace));

    with_default(subscriber, || {
        foo();
    });

    shutdown_tracer_provider();
    // <--- App stuck here calling `TracerProviderInner::drop`
    // which calls `futures_executor::block_on(res_receiver)` in `BatchSpanProcessor::shutdown`.
    // Remove `shutdown_tracer_provider` to avoid the app stuck but still not generating jaeger trace.
}

#[tokio::test]
async fn subscriber_set_default() {
    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
            Resource::new(vec![KeyValue::new("service.name", "issue-02")]),
        ))
        .install_batch(runtime::Tokio)
        .unwrap();
    global::set_tracer_provider(provider.clone());

    let trace = provider.tracer("issue-02");

    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(trace));

    let _guard = set_default(subscriber);

    foo();

    shutdown_tracer_provider();

    // <--- App stuck here calling `TracerProviderInner::drop`
    // which calls `futures_executor::block_on(res_receiver)` in `BatchSpanProcessor::shutdown`
    // Remove `shutdown_tracer_provider` to avoid the app stuck but still not generating jaeger trace.
}
