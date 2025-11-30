pub mod data_processing;
pub mod report_generator;
pub mod data_models;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_data_processing() {
        let processor = data_processing::DataProcessor::new();
        let sample_data = processor.generate_sample_data().await;
        assert!(!sample_data.is_empty());

        let transformed = processor.transform_data(sample_data);
        assert!(transformed.summary.total_metrics > 0);
    }

    #[test]
    fn test_report_generation() {
        use data_models::*;
        use std::collections::HashMap;

        let generator = report_generator::ReportGenerator::new();
        let test_data = TransformedData {
            summary: DataSummary {
                total_metrics: 100,
                average_value: 50.5,
                top_performing_category: "Sales".to_string(),
                regional_breakdown: HashMap::from([
                    ("North America".to_string(), 1000.0),
                    ("Europe".to_string(), 800.0),
                ]),
            },
            trends: vec![],
            comparisons: HashMap::new(),
        };

        let result = generator.generate_excel_report(&test_data, "test_report.xlsx");
        assert!(result.is_ok());
    }
}