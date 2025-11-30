mod data_processing;
mod report_generator;
mod data_models;

use data_processing::DataProcessor;
use report_generator::ReportGenerator;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Nintendo Intern Reporting Tool Backend Started");

    // Initialize processors
    let data_processor = DataProcessor::new();
    let report_generator = ReportGenerator::new();

    // Process sample data and generate report
    let sample_data = data_processor.generate_sample_data().await;
    println!("Generated sample data with {} records", sample_data.len());

    // Transform data for reporting
    let transformed_data = data_processor.transform_data(sample_data);
    println!("Transformed data for reporting");

    // Print some sample data for data creation confirmation
    println!("Sample Summary - Total Metrics: {}, Average Value: {:.2}",
             transformed_data.summary.total_metrics,
             transformed_data.summary.average_value);

    // Generate Excel report
    let filename = "business_report.xlsx";
    println!("Attempting to generate Excel report: {}", filename);

    report_generator.generate_excel_report(&transformed_data, filename)
        .map_err(|e| {
            println!("Error generating Excel report: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;

    // Check if .xlsx file was created
    if Path::new(filename).exists() {
        println!("✅ Excel report successfully created: {}", filename);
        println!("📊 File location: {}", std::env::current_dir()?.display());
    } else {
        println!("❌ Excel file was not created!");
    }

    // Demonstrate JSON output for potential frontend
    let json_data = report_generator.generate_json_data(&transformed_data)?;
    println!("Sample JSON data (for frontend):\n{}", json_data);

    Ok(())
}