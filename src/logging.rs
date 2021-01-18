use tracing::Dispatch;
use tracing_timing::{Builder, Histogram};

pub fn setup_logging() {
    setup_simple();
    setup_tracing_timing();
}

fn setup_tracing_timing() {
    let subscriber = Builder::default().build(|| Histogram::new_with_max(1_000_000, 2).unwrap());
    let dispatcher = Dispatch::new(subscriber);
    tracing::dispatcher::set_global_default(dispatcher)
        .expect("global default was already set!");
}

fn setup_simple() {
    simplelog::SimpleLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::ConfigBuilder::new()
            // .add_filter_allow_str("balz")
            .set_time_format("%H:%M:%S%.3f".to_string())
            .build(),
    )
        .expect("could not setup logging");
}