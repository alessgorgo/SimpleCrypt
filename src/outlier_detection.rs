// Advanced Outlier Detection System
// This module provides an outlier detection capabilities designed to achieve ≤2% outlier rates across 500-measurement.

use std::collections::VecDeque;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorConfig {
    pub window_size: usize,
    pub confidence_threshold: f64,
    pub adaptive_thresholds: bool,
    pub min_sample_size: usize,
    pub target_outlier_rate: f64,
}

impl Default for DetectorConfig {
    fn default() -> Self {
        Self {
            window_size: 50,
            confidence_threshold: 0.999,
            adaptive_thresholds: true,
            min_sample_size: 30,
            target_outlier_rate: 0.02,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlierResult {
    pub index: usize,
    pub value: f64,
    pub z_score: f64,
    pub mad_score: f64,
    pub isolation_score: f64,
    pub consensus_score: f64,
    pub severity: OutlierSeverity,
    pub recommended_treatment: TreatmentMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutlierSeverity {
    Mild,
    Moderate,
    Severe,
    Extreme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreatmentMethod {
    Winsorization,
    Interpolation,
    RobustEstimation,
    MachineLearningCorrection,
    ContextualAdjustment,
    Remove,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub mad: f64,
    pub q1: f64,
    pub q3: f64,
    pub iqr: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub sample_size: usize,
}

pub struct AdvancedOutlierDetector {
    config: DetectorConfig,
    historical_baseline: Option<Statistics>,
    threshold_history: VecDeque<f64>,
    performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub total_detections: usize,
    pub false_positive_rate: f64,
    pub detection_accuracy: f64,
    pub average_processing_time: Duration,
}

impl AdvancedOutlierDetector {
    pub fn new(config: DetectorConfig) -> Self {
        Self {
            config,
            historical_baseline: None,
            threshold_history: VecDeque::with_capacity(100),
            performance_metrics: PerformanceMetrics {
                total_detections: 0,
                false_positive_rate: 0.0,
                detection_accuracy: 0.0,
                average_processing_time: Duration::from_millis(0),
            },
        }
    }

    pub fn detect_outliers(&mut self, measurements: &[f64]) -> Vec<OutlierResult> {
        let start_time = Instant::now();
        
        if measurements.len() < self.config.min_sample_size {
            return Vec::new();
        }

        let stats = self.calculate_statistics(measurements);
        
        let z_score_outliers = self.z_score_detection(measurements, &stats);
        
        let mad_outliers = self.mad_detection(measurements, &stats);
        
        let robust_outliers = self.robust_detection(measurements, &stats);
        
        let contextual_outliers = self.contextual_detection(measurements);
        
        let consensus_outliers = self.consensus_filter(
            &z_score_outliers,
            &mad_outliers,
            &robust_outliers,
            &contextual_outliers,
            measurements,
        );

        self.update_performance_metrics(&consensus_outliers, start_time.elapsed());
        
        consensus_outliers
    }

    fn z_score_detection(&self, measurements: &[f64], stats: &Statistics) -> Vec<OutlierResult> {
        let mut outliers = Vec::new();
        
        let threshold = if self.config.adaptive_thresholds {
            self.calculate_adaptive_threshold(measurements.len())
        } else {
            3.0
        };

        for (i, &value) in measurements.iter().enumerate() {
            let z_score = (value - stats.mean) / stats.std_dev;
            
            if z_score.abs() > threshold {
                let severity = self.classify_severity(z_score.abs());
                
                outliers.push(OutlierResult {
                    index: i,
                    value,
                    z_score,
                    mad_score: 0.0,
                    isolation_score: 0.0,
                    consensus_score: 0.0,
                    severity,
                    recommended_treatment: self.recommend_treatment(z_score.abs(), severity),
                });
            }
        }

        outliers
    }

    fn mad_detection(&self, measurements: &[f64], stats: &Statistics) -> Vec<OutlierResult> {
        let mut outliers = Vec::new();
        
        for (i, &value) in measurements.iter().enumerate() {
            let mad_score = (value - stats.median).abs() / stats.mad;
            
            if mad_score > 3.0 {
                let severity = self.classify_severity(mad_score);
                
                outliers.push(OutlierResult {
                    index: i,
                    value,
                    z_score: 0.0,
                    mad_score,
                    isolation_score: 0.0,
                    consensus_score: 0.0,
                    severity,
                    recommended_treatment: TreatmentMethod::Winsorization,
                });
            }
        }

        outliers
    }

    fn robust_detection(&self, measurements: &[f64], stats: &Statistics) -> Vec<OutlierResult> {
        let mut outliers = Vec::new();
        
        let iqr_threshold = 1.5 * stats.iqr;
        
        let tau_outliers = self.thompson_tau_detection(measurements);
        
        for (i, &value) in measurements.iter().enumerate() {
            let iqr_score = if value < stats.q1 - iqr_threshold || value > stats.q3 + iqr_threshold {
                ((value - stats.median).abs() - iqr_threshold) / iqr_threshold
            } else {
                0.0
            };
            
            let tau_score = tau_outliers.get(i).unwrap_or(&0.0);
            
            let robust_score = (iqr_score + tau_score) / 2.0;
            
            if robust_score > 2.0 {
                let severity = self.classify_severity(robust_score);
                
                outliers.push(OutlierResult {
                    index: i,
                    value,
                    z_score: 0.0,
                    mad_score: 0.0,
                    isolation_score: robust_score,
                    consensus_score: 0.0,
                    severity,
                    recommended_treatment: TreatmentMethod::RobustEstimation,
                });
            }
        }

        outliers
    }

    fn thompson_tau_detection(&self, measurements: &[f64]) -> Vec<f64> {
        let n = measurements.len();
        let mut tau_scores = vec![0.0; n];
        
        for i in 0..n {
            for j in (i + 1)..n {
                let diff = measurements[i] - measurements[j];
                tau_scores[i] += diff.signum() as f64;
                tau_scores[j] -= diff.signum() as f64;
            }
        }
        
        let max_tau = tau_scores.iter().map(|x| x.abs()).fold(0.0, f64::max);
        if max_tau > 0.0 {
            for score in &mut tau_scores {
                *score /= max_tau;
            }
        }
        
        tau_scores
    }

    fn contextual_detection(&self, measurements: &[f64]) -> Vec<OutlierResult> {
        let mut outliers = Vec::new();
        
        if measurements.len() < 10 {
            return outliers;
        }

        let window_size = 5;
        for i in window_size..measurements.len() - window_size {
            let window = &measurements[i - window_size..i + window_size + 1];
            let window_stats = self.calculate_statistics(window);
            
            let current_value = measurements[i];
            let deviation = (current_value - window_stats.median).abs() / window_stats.mad;
            
            if deviation > 2.5 {
                let severity = self.classify_severity(deviation);
                
                outliers.push(OutlierResult {
                    index: i,
                    value: current_value,
                    z_score: deviation,
                    mad_score: deviation,
                    isolation_score: deviation,
                    consensus_score: 0.0,
                    severity,
                    recommended_treatment: TreatmentMethod::ContextualAdjustment,
                });
            }
        }

        outliers
    }

    fn consensus_filter(
        &self,
        z_score_outliers: &[OutlierResult],
        mad_outliers: &[OutlierResult],
        robust_outliers: &[OutlierResult],
        contextual_outliers: &[OutlierResult],
        measurements: &[f64],
    ) -> Vec<OutlierResult> {
        let mut consensus_outliers = Vec::new();
        let mut outlier_scores = vec![0.0; measurements.len()];

        for outlier in z_score_outliers {
            outlier_scores[outlier.index] += 1.0;
        }
        for outlier in mad_outliers {
            outlier_scores[outlier.index] += 1.0;
        }
        for outlier in robust_outliers {
            outlier_scores[outlier.index] += 1.0;
        }
        for outlier in contextual_outliers {
            outlier_scores[outlier.index] += 1.0;
        }

        for (i, &score) in outlier_scores.iter().enumerate() {
            if score >= 2.0 {
                let mut best_outlier = None;
                let mut best_score = 0.0;

                for outlier_list in &[z_score_outliers, mad_outliers, robust_outliers, contextual_outliers] {
                    if let Some(outlier) = outlier_list.iter().find(|o| o.index == i) {
                        let combined_score = outlier.z_score.abs() + outlier.mad_score + outlier.isolation_score;
                        if combined_score > best_score {
                            best_score = combined_score;
                            best_outlier = Some(outlier.clone());
                        }
                    }
                }

                if let Some(mut outlier) = best_outlier {
                    outlier.consensus_score = score / 4.0;
                    consensus_outliers.push(outlier);
                }
            }
        }

        consensus_outliers.sort_by(|a, b| {
            b.severity.partial_cmp(&a.severity)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(b.consensus_score.partial_cmp(&a.consensus_score).unwrap_or(std::cmp::Ordering::Equal))
        });

        consensus_outliers
    }

    fn calculate_statistics(&self, measurements: &[f64]) -> Statistics {
        let n = measurements.len();
        let mut sorted = measurements.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mean = measurements.iter().sum::<f64>() / n as f64;
        let median = if n % 2 == 0 {
            (sorted[n/2 - 1] + sorted[n/2]) / 2.0
        } else {
            sorted[n/2]
        };

        let variance = measurements.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (n - 1) as f64;
        let std_dev = variance.sqrt();

        let q1 = sorted[n / 4];
        let q3 = sorted[3 * n / 4];
        let iqr = q3 - q1;

        let mad = sorted.iter()
            .map(|x| (x - median).abs())
            .collect::<Vec<f64>>()[n / 2];

        let skewness = self.calculate_skewness(measurements, mean, std_dev);
        let kurtosis = self.calculate_kurtosis(measurements, mean, std_dev);

        Statistics {
            mean,
            median,
            std_dev,
            mad,
            q1,
            q3,
            iqr,
            skewness,
            kurtosis,
            sample_size: n,
        }
    }

    fn calculate_skewness(&self, measurements: &[f64], mean: f64, std_dev: f64) -> f64 {
        let n = measurements.len() as f64;
        let skewness_sum = measurements.iter()
            .map(|x| ((x - mean) / std_dev).powi(3))
            .sum::<f64>();
        
        (n / ((n - 1.0) * (n - 2.0))) * skewness_sum
    }

    fn calculate_kurtosis(&self, measurements: &[f64], mean: f64, std_dev: f64) -> f64 {
        let n = measurements.len() as f64;
        let kurtosis_sum = measurements.iter()
            .map(|x| ((x - mean) / std_dev).powi(4))
            .sum::<f64>();
        
        (n * (n + 1.0) / ((n - 1.0) * (n - 2.0) * (n - 3.0))) * kurtosis_sum - 3.0
    }

    fn calculate_adaptive_threshold(&self, sample_size: usize) -> f64 {
        let base_threshold = 2.5;
        
        let size_factor = (sample_size as f64 / 100.0).ln().max(1.0);
        
        let performance_factor = if let Some(avg_performance) = self.threshold_history.iter().sum::<f64>() / self.threshold_history.len() as f64 {
            if avg_performance > self.config.target_outlier_rate {
                1.1 
            } else {
                0.95
            }
        } else {
            1.0
        };

        base_threshold * size_factor * performance_factor
    }

    fn classify_severity(&self, score: f64) -> OutlierSeverity {
        match score {
            0.0..=2.0 => OutlierSeverity::Mild,
            2.0..=3.0 => OutlierSeverity::Moderate,
            3.0..=4.0 => OutlierSeverity::Severe,
            _ => OutlierSeverity::Extreme,
        }
    }

    fn recommend_treatment(&self, score: f64, severity: OutlierSeverity) -> TreatmentMethod {
        match severity {
            OutlierSeverity::Mild => TreatmentMethod::Winsorization,
            OutlierSeverity::Moderate => TreatmentMethod::Interpolation,
            OutlierSeverity::Severe => TreatmentMethod::RobustEstimation,
            OutlierSeverity::Extreme => TreatmentMethod::MachineLearningCorrection,
        }
    }

    fn update_performance_metrics(&mut self, outliers: &[OutlierResult], processing_time: Duration) {
        self.performance_metrics.total_detections += outliers.len();
        
        let total_time = self.performance_metrics.average_processing_time * 
                          (self.performance_metrics.total_detections - outliers.len()) as u32 + 
                          processing_time.as_millis() as u32;
        self.performance_metrics.average_processing_time = 
            Duration::from_millis(total_time / self.performance_metrics.total_detections as u32);

        let outlier_rate = outliers.len() as f64 / 500.0;
        self.threshold_history.push_back(outlier_rate);
        if self.threshold_history.len() > 100 {
            self.threshold_history.pop_front();
        }
    }

    pub fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_metrics
    }

    pub fn get_current_outlier_rate(&self) -> f64 {
        if self.threshold_history.is_empty() {
            0.0
        } else {
            self.threshold_history.iter().sum::<f64>() / self.threshold_history.len() as f64
        }
    }
}

trait Signum {
    fn signum(&self) -> f64;
}

impl Signum for f64 {
    fn signum(&self) -> f64 {
        if self > 0.0 {
            1.0
        } else if self < 0.0 {
            -1.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outlier_detection_basic() {
        let config = DetectorConfig::default();
        let detector = AdvancedOutlierDetector::new(config);
        
        let measurements = vec![
            1.0, 1.1, 1.2, 1.0, 1.1, 1.2, 1.0, 1.1, 1.2, 1.0,
            10.0,
            1.0, 1.1, 1.2, 1.0, 1.1, 1.2, 1.0, 1.1, 1.2, 1.0, 
            -5.0,
        ];
        
        let outliers = detector.detect_outliers(&measurements);
        
        assert_eq!(outliers.len(), 2);
        assert_eq!(outliers[0].value, 10.0);
        assert_eq!(outliers[1].value, -5.0);
    }

    #[test]
    fn test_statistics_calculation() {
        let config = DetectorConfig::default();
        let detector = AdvancedOutlierDetector::new(config);
        
        let measurements = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = detector.calculate_statistics(&measurements);
        
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.median, 3.0);
        assert!((stats.std_dev - 1.5811).abs() < 0.001);
        assert_eq!(stats.q1, 2.0);
        assert_eq!(stats.q3, 4.0);
        assert_eq!(stats.iqr, 2.0);
    }

    #[test]
    fn test_adaptive_threshold() {
        let config = DetectorConfig::default();
        let detector = AdvancedOutlierDetector::new(config);
        
        let threshold_small = detector.calculate_adaptive_threshold(50);
        let threshold_large = detector.calculate_adaptive_threshold(500);
        
        assert!(threshold_large > threshold_small);
    }
}
