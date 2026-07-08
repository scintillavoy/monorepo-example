use tracing::level_filters::LevelFilter;

pub struct OtelConfig {
    pub log_level: LevelFilter,
    pub service_name: String,
    pub endpoint: String,
    pub custom_attribute: String,
    pub trace_sampling_ratio: f64,
}
