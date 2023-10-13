use tracing_subscriber::{filter, prelude::*};

pub fn init() {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let file_appender = tracing_appender::rolling::daily("log", "web-demo.log");
    let log_layer = tracing_subscriber::fmt::layer().with_writer(file_appender);


    tracing_subscriber::registry()
        .with(
            stdout_log
                // Add an `INFO` filter to the stdout logging layer
                .with_filter(filter::LevelFilter::INFO)
                // Combine the filtered `stdout_log` layer with the
                // `debug_log` layer, producing a new `Layered` layer.
                .and_then(log_layer)
                // Add a filter to *both* layers that rejects spans and
                // events whose targets start with `metrics`.
                .with_filter(filter::filter_fn(|metadata| {
                    !metadata.target().starts_with("metrics")
                }))
        )
        .init();
}