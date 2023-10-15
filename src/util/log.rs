use tracing_subscriber::prelude::*;
use tracing_appender::rolling;

pub fn init() {
    let info_file = rolling::daily("log", "info");
    let err_file = rolling::daily("log", "error").with_max_level(tracing::Level::ERROR);
    let all_files = info_file.and(err_file);

    tracing_subscriber::fmt()
        .with_writer(all_files)
        .with_line_number(true)
        .with_ansi(false)
        // .with_max_level(tracing::Level::TRACE)
    .init();


    // let stdout_log = tracing_subscriber::fmt::layer().pretty();
    // let file_appender = tracing_appender::rolling::daily("log", "web-demo.log");
    // let log_layer = tracing_subscriber::fmt::layer().with_writer(file_appender);
    //
    //
    // tracing_subscriber::registry()
    //     .with(
    //         stdout_log
    //             // Add an `INFO` filter to the stdout logging layer
    //             .with_filter(filter::LevelFilter::INFO)
    //             // Combine the filtered `stdout_log` layer with the
    //             // `debug_log` layer, producing a new `Layered` layer.
    //             // Add a filter to *both* layers that rejects spans and
    //             // events whose targets start with `metrics`.
    //             .with_filter(filter::filter_fn(|metadata| {
    //                 !metadata.target().starts_with("metrics")
    //             }))
    //             .and_then(log_layer)
    //     )
    //     .init();
}