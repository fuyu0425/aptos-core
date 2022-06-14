// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use log::warn;
use prometheus_parse::Scrape as PrometheusScrape;
use prometheus_parse::Value as PrometheusValue;

pub fn get_metric_value(
    metrics: &PrometheusScrape,
    metric_name: &str,
    label_key: &str,
    label_value: &str,
) -> Option<u64> {
    for s in &metrics.samples {
        if s.metric == metric_name {
            let lv = s.labels.get(label_key);
            if lv.is_none() {
                continue;
            }
            if lv.unwrap() != label_value {
                continue;
            }
            match &s.value {
                PrometheusValue::Counter(v) => return Some(v.round() as u64),
                PrometheusValue::Gauge(v) => return Some(v.round() as u64),
                PrometheusValue::Untyped(v) => return Some(v.round() as u64),
                wildcard => {
                    warn!("Found unexpected metric type: {:?}", wildcard);
                }
            }
        }
    }
    None
}
