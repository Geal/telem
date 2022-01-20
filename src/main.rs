mod std_exporter;
use opentelemetry::trace::Tracer;
use std_exporter::new_pipeline;
use tracing::{instrument, span};

#[instrument(name = "do nothing", level = "error")]
async fn do_nothing(doing: &str) {
    tracing::info!("huh");
    eprintln!("doing {}", doing);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt().json().init();

    loop {
        let tracer = new_pipeline().with_pretty_print(true).install_batch();
        let root = span!(tracing::Level::INFO, "app_start", work_units = 2);
        let _entry = root.enter();

        tracer
            .in_span("doing_work", |_cx| async {
                // Traced app logic here...
                do_nothing("with context").await
            })
            .await;

        tracing::info!("hmm");
        do_nothing("with it").await;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    // Shutdown trace pipeline
    // Ok(global::shutdown_tracer_provider())
}
