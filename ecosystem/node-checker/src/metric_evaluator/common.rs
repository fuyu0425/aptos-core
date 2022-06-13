use prometheus_parse::Scrape as PrometheusScrape;
use prometheus_parse::Value as PrometheusValue;

pub fn get_metric_value(metrics: &PrometheusScrape, metric_name: &str) -> Option<u64> {
    for s in &metrics.samples {
        if s.metric == metric_name {
            if let PrometheusValue::Untyped(v) = s.value {
                return Some(v.round() as u64);
            }
        }
    }
    None
}
