# issue_opentelemetry_jaeger

These sets of crates provide a reproducible demo of the issue with OpenTelemetry Rust SDK and Jaeger.

To reproduce the issue, run the following commands first:

```bash
$ docker run -d -p16686:16686 -p4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest
```

Folder structure:

- `01-legacy`: This crate contains an example that successfully exports traces to Jaeger with old versions.
- `02-latest`: This crate contains an example that fails to export traces to Jaeger with the latest versions.

All the demo code is using `tracing-opentelemetry` and `opentelemetry-*` to export the traces to Jaeger via otlp-grpc.
