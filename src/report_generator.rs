use crate::data_models::TransformedData;
use std::collections::HashMap;
use rust_xlsxwriter::{Workbook, Format, XlsxError};

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generate Excel report with business insights
    pub fn generate_excel_report(&self, data: &TransformedData, filename: &str) -> Result<(), XlsxError> {
        println!("Generating Excel report: {}", filename);

        // Create a new workbook
        let mut workbook = Workbook::new();

        // Summary sheet
        let mut worksheet = workbook.add_worksheet();
        worksheet.set_name("Summary")?;
        self.create_summary_sheet(&mut worksheet, &data.summary)?;

        // Trends sheet
        let mut worksheet = workbook.add_worksheet();
        worksheet.set_name("Trends")?;
        self.create_trends_sheet(&mut worksheet, &data.trends)?;

        // Comparisons sheet
        let mut worksheet = workbook.add_worksheet();
        worksheet.set_name("Comparisons")?;
        self.create_comparisons_sheet(&mut worksheet, &data.comparisons)?;

        // Save workbook
        workbook.save(filename)?;
        println!("Excel report saved: {}", filename);

        Ok(())
    }

    fn create_summary_sheet(&self, worksheet: &mut rust_xlsxwriter::Worksheet, summary: &crate::data_models::DataSummary) -> Result<(), XlsxError> {
        // Create header format
        let header_format = Format::new().set_bold();

        // Headers with formatting
        worksheet.write_string_with_format(0, 0, "Business Performance Summary", &header_format)?;
        worksheet.write_string_with_format(1, 0, "Metric", &header_format)?;
        worksheet.write_string_with_format(1, 1, "Value", &header_format)?;

        // Data
        worksheet.write_string(2, 0, "Total Metrics Tracked")?;
        worksheet.write_number(2, 1, summary.total_metrics as f64)?;

        worksheet.write_string(3, 0, "Average Metric Value")?;
        worksheet.write_number(3, 1, summary.average_value)?;

        worksheet.write_string(4, 0, "Top Performing Category")?;
        worksheet.write_string(4, 1, &summary.top_performing_category)?;

        // Regional Breakdown
        worksheet.write_string_with_format(6, 0, "Regional Breakdown", &header_format)?;
        worksheet.write_string_with_format(7, 0, "Region", &header_format)?;
        worksheet.write_string_with_format(7, 1, "Total Value", &header_format)?;

        let mut row = 8;
        for (region, value) in &summary.regional_breakdown {
            worksheet.write_string(row, 0, region)?;
            worksheet.write_number(row, 1, *value)?;
            row += 1;
        }

        println!("Summary sheet created with {} regional entries", summary.regional_breakdown.len());
        Ok(())
    }

    fn create_trends_sheet(&self, worksheet: &mut rust_xlsxwriter::Worksheet, trends: &[crate::data_models::DataTrend]) -> Result<(), XlsxError> {
        // Create header format
        let header_format = Format::new().set_bold();

        // Headers with formatting
        worksheet.write_string_with_format(0, 0, "Business Trends Over Time", &header_format)?;
        worksheet.write_string_with_format(1, 0, "Period", &header_format)?;
        worksheet.write_string_with_format(1, 1, "Value", &header_format)?;
        worksheet.write_string_with_format(1, 2, "Trend Direction", &header_format)?;

        // Data
        for (i, trend) in trends.iter().enumerate() {
            let row = (i + 2) as u32;
            worksheet.write_string(row, 0, &trend.period)?;
            worksheet.write_number(row, 1, trend.value)?;

            let direction = match trend.trend_direction {
                crate::data_models::TrendDirection::Increasing => "Increasing",
                crate::data_models::TrendDirection::Decreasing => "Decreasing",
                crate::data_models::TrendDirection::Stable => "Stable",
            };
            worksheet.write_string(row, 2, direction)?;
        }

        println!("Trends sheet created with {} trend entries", trends.len());
        Ok(())
    }

    fn create_comparisons_sheet(&self, worksheet: &mut rust_xlsxwriter::Worksheet, comparisons: &HashMap<String, crate::data_models::ComparisonData>) -> Result<(), XlsxError> {
        // Create header format
        let header_format = Format::new().set_bold();

        // Headers with formatting
        worksheet.write_string_with_format(0, 0, "Performance Comparisons", &header_format)?;
        worksheet.write_string_with_format(1, 0, "Metric", &header_format)?;
        worksheet.write_string_with_format(1, 1, "Current", &header_format)?;
        worksheet.write_string_with_format(1, 2, "Previous", &header_format)?;
        worksheet.write_string_with_format(1, 3, "Change %", &header_format)?;
        worksheet.write_string_with_format(1, 4, "Status", &header_format)?;

        // Data
        for (i, (metric, comparison)) in comparisons.iter().enumerate() {
            let row = (i + 2) as u32;
            worksheet.write_string(row, 0, metric)?;
            worksheet.write_number(row, 1, comparison.current)?;
            worksheet.write_number(row, 2, comparison.previous)?;
            worksheet.write_number(row, 3, comparison.change_percentage)?;

            let status = if comparison.change_percentage > 0.0 {
                "Improving"
            } else if comparison.change_percentage < 0.0 {
                "Declining"
            } else {
                "Stable"
            };
            worksheet.write_string(row, 4, status)?;
        }

        println!("Comparisons sheet created with {} comparison entries", comparisons.len());
        Ok(())
    }

    /// Generate JSON data for frontend visualization
    pub fn generate_json_data(&self, data: &TransformedData) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(data)
    }
}