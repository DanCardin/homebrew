use tracing::{subscriber::set_global_default, Subscriber};
// use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber(env_filter: String) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let fmt_layer = fmt::layer().json();

    Registry::default().with(env_filter).with(fmt_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
