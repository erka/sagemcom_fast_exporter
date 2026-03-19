use prometheus::{Encoder, Gauge, Histogram, HistogramOpts, Registry, TextEncoder};
use std::time::Instant;
use tikv_jemalloc_ctl::{epoch, stats};

pub trait AllocatorStats: Send + Sync {
    fn sys_bytes(&self) -> Option<u64>;
}

struct JemallocStats {
    initialized: std::sync::OnceLock<()>,
}

impl JemallocStats {
    fn new() -> Self {
        Self {
            initialized: std::sync::OnceLock::new(),
        }
    }

    fn ensure_initialized(&self) {
        self.initialized.get_or_init(|| {
            let _ = epoch::advance();
        });
    }
}

impl AllocatorStats for JemallocStats {
    fn sys_bytes(&self) -> Option<u64> {
        self.ensure_initialized();
        stats::mapped::read().ok().map(|v| v as u64)
    }
}

pub struct ExporterMetrics {
    registry: Registry,
    scrape_duration_seconds: Histogram,
    scrape_success: Gauge,
    allocator_sys_bytes: Gauge,
    allocator: Box<dyn AllocatorStats>,
    scrape_timer: Option<Instant>,
}

impl ExporterMetrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let scrape_duration_seconds = Histogram::with_opts(
            HistogramOpts::new(
                "sagemcom_fast_exporter_collector_scrape_duration_seconds",
                "Duration of a collector scrape.",
            )
            .buckets(vec![
                0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
            ]),
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(scrape_duration_seconds.clone()))
            .expect("failed to create metric");

        let scrape_success = Gauge::new(
            "sagemcom_fast_exporter_collector_scrape_success",
            "Whether a collection succeeded.",
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(scrape_success.clone()))
            .expect("failed to create metric");

        let allocator_sys_bytes = Gauge::new(
            "sagemcom_fast_exporter_allocator_sys_bytes",
            "Allocator obtained bytes (jemalloc)",
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(allocator_sys_bytes.clone()))
            .expect("failed to create metric");

        Self {
            registry,
            scrape_duration_seconds,
            scrape_success,
            allocator_sys_bytes,
            allocator: Box::new(JemallocStats::new()),
            scrape_timer: None,
        }
    }

    pub fn update_allocator_stats(&mut self) {
        self.allocator_sys_bytes
            .set(self.allocator.sys_bytes().map(|v| v as f64).unwrap_or(0.0));
    }

    pub fn start_scrape_timer(&mut self) {
        self.scrape_timer = Some(Instant::now());
    }

    pub fn observe_scrape(&mut self, success: bool) {
        if let Some(start) = self.scrape_timer.take() {
            let duration = start.elapsed().as_secs_f64();
            self.scrape_duration_seconds.observe(duration);
        }
        self.scrape_success.set(if success { 1.0 } else { 0.0 });
    }

    pub fn gather(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder
            .encode(&metric_families, &mut buffer)
            .expect("failed to create metric");
        String::from_utf8(buffer).expect("failed to create metric")
    }
}

impl Default for ExporterMetrics {
    fn default() -> Self {
        Self::new()
    }
}
