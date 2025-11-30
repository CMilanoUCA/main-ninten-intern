use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessData {
    pub id: String,
    pub timestamp: String,
    pub category: String,
    pub metric_name: String,
    pub value: f64,
    pub region: String,
    pub department: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformedData {
    pub summary: DataSummary,
    pub trends: Vec<DataTrend>,
    pub comparisons: HashMap<String, ComparisonData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSummary {
    pub total_metrics: usize,
    pub average_value: f64,
    pub top_performing_category: String,
    pub regional_breakdown: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTrend {
    pub period: String,
    pub value: f64,
    pub trend_direction: TrendDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonData {
    pub current: f64,
    pub previous: f64,
    pub change_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRequest {
    pub start_date: String,
    pub end_date: String,
    pub categories: Vec<String>,
    pub regions: Vec<String>,
    pub metrics: Vec<String>,
    pub report_type: ReportType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportType {
    Summary,
    Detailed,
    Comparative,
    Custom,
}