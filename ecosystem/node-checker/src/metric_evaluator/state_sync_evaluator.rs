use super::common::get_metric_value;
use super::{MetricsEvaluator, MetricsEvaluatorError};
use crate::public_types::Evaluation;
use anyhow::Result;
use log::debug;
use prometheus_parse::Scrape as PrometheusScrape;
use prometheus_parse::Value as PrometheusValue;

const STATE_SYNC_METRIC: &str = "aptos_state_sync_version";

#[derive(Clone, Debug)]
struct StateSyncMetricsEvaluator {
    version_delta_tolerance: u64,
}

impl StateSyncMetricsEvaluator {
    pub fn new(version_delta_tolerance: u64) -> Self {
        Self {
            version_delta_tolerance,
        }
    }

    fn get_sync_version(&self, metrics: &PrometheusScrape) -> Option<u64> {
        get_metric_value(&metrics, STATE_SYNC_METRIC)
    }
}

impl MetricsEvaluator for StateSyncMetricsEvaluator {
    /// Assert that the state sync version is increasing on the target node
    /// and that we're within tolerance of the baseline node's latest version.
    fn evaluate_metrics(
        &self,
        previous_baseline_metrics: &PrometheusScrape,
        previous_target_metrics: &PrometheusScrape,
        latest_baseline_metrics: &PrometheusScrape,
        latest_target_metrics: &PrometheusScrape,
    ) -> Result<Vec<Evaluation>, MetricsEvaluatorError> {
        let mut evaluations = vec![];

        // Get previous version from the target node.
        let previous_target_version = self.get_sync_version(previous_target_metrics);

        if previous_target_version.is_none() {
            evaluations.push(Evaluation {
                headline: "State sync version metric missing".to_string(),
                score: 0,
                explanation:
                    "The first set of metrics from the target node is missing the state sync metric"
                        .to_string(),
            });
        }

        // Get the latest version from the target node.
        let latest_target_version = self.get_sync_version(latest_target_metrics);

        if latest_target_version.is_none() {
            evaluations.push(Evaluation {
                headline: "State sync version metric missing".to_string(),
                score: 0,
                explanation: "The second set of metrics from the target node is missing the state sync metric".to_string(),
            });
        }

        // Get the latest state sync version from the baseline node.
        let latest_baseline_version = self
            .get_sync_version(latest_baseline_metrics)
            .ok_or(MetricsEvaluatorError::MissingBaselineMetric(
            STATE_SYNC_METRIC.to_string(),
            "The latest set of metrics from the baseline node did not contain the necessary key"
                .to_string(),
        ))?;

        match (previous_target_version, latest_target_version) {
            (Some(previous), Some(latest)) => {
                let primary_evaluation = {
                    let target_progress = latest - previous;
                    if (target_progress) == 0 {
                        Evaluation{
                        headline: "State sync version is not progressing".to_string(),
                        score: 50,
                        explanation: "Successfully pulled metrics from target node twice, but the metrics aren't progressing".to_string(),
                    }
                    } else {
                        let delta_from_baseline = latest_baseline_version - latest;
                        if delta_from_baseline > self.version_delta_tolerance {
                            Evaluation {
                                headline: "State sync version is lagging".to_string(),
                                score: 70,
                                explanation: format!(
                                    "Successfully pulled metrics from target node twice and saw the \
                                    version was progressing, but it is lagging {} versions behind the baseline node. \
                                    Target version: {}. Baseline version: {}. Tolerance: {}",
                                    delta_from_baseline, latest, latest_baseline_version, self.version_delta_tolerance
                                ),
                            }
                        } else {
                            Evaluation {
                                headline: "State sync version is within tolerance".to_string(),
                                score: 100,
                                explanation: format!(
                                    "Successfully pulled metrics from target node twice, saw the \
                                    version was progressing, and saw that it is within tolerance \
                                    of the baseline node. \
                                    Target version: {}. Baseline version: {}. Tolerance: {}",
                                    latest, latest_baseline_version, self.version_delta_tolerance
                                ),
                            }
                        }
                    }
                };
                evaluations.push(primary_evaluation);
            }
            _ => {
                debug!("Not evaluating state sync version because we're missing metrics from the target");
            }
        };

        Ok(evaluations)
    }

    fn get_name() -> String {
        "State Sync".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_metric_string(value: u64) -> String {
        let mut s = r#"aptos_state_sync_version{type="synced"} "#.to_string();
        s.push_str(&format!("{}", value));
        s
    }

    fn test_state_sync_metrics_evaluator(
        previous_target_version: u64,
        latest_baseline_version: u64,
        latest_target_version: u64,
        expected_score: u8,
        fail_on_purpose: bool,
    ) {
        let previous_baseline_metrics = vec![get_metric_string(0)]; // This one doesn't matter right now.
        let previous_target_metrics = vec![get_metric_string(previous_target_version)];
        let latest_baseline_metrics = vec![get_metric_string(latest_baseline_version)];

        let latest_target_metrics = match fail_on_purpose {
            true => vec![],
            false => vec![get_metric_string(latest_target_version)],
        };

        let state_sync_metrics_evaluator = StateSyncMetricsEvaluator::new(1000);
        let evaluations = state_sync_metrics_evaluator
            .evaluate_metrics(
                &PrometheusScrape::parse(
                    previous_baseline_metrics.iter().map(|l| Ok(l.to_string())),
                )
                .unwrap(),
                &PrometheusScrape::parse(previous_target_metrics.iter().map(|l| Ok(l.to_string())))
                    .unwrap(),
                &PrometheusScrape::parse(latest_baseline_metrics.iter().map(|l| Ok(l.to_string())))
                    .unwrap(),
                &PrometheusScrape::parse(latest_target_metrics.iter().map(|l| Ok(l.to_string())))
                    .unwrap(),
            )
            .expect("Failed to evaluate metrics");

        assert_eq!(evaluations.len(), 1);
        assert_eq!(evaluations[0].score, expected_score);
    }

    #[test]
    fn test_in_sync_and_progressing() {
        test_state_sync_metrics_evaluator(1000, 2000, 1700, 100, false);
    }

    #[test]
    fn test_progressing_but_lagging() {
        test_state_sync_metrics_evaluator(1000, 5000, 3000, 70, false);
    }

    #[test]
    fn test_not_progressing() {
        test_state_sync_metrics_evaluator(1000, 5000, 1000, 50, false);
    }

    #[test]
    fn test_missing_metric() {
        test_state_sync_metrics_evaluator(1000, 5000, 1000, 0, true);
    }
}
