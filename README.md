# Nintendo Internship Business Reporting Tool

A Rust-based internal reporting tool that extracts, transforms, and visualizes business data with Excel report generation. Built for the Nintendo Software Engineer Internship application within JetBrains RustRover IDE.

## Main Features

- **Data Extraction & Transformation**: Generates realistic business metrics and transforms them into meaningful insights
- **Excel Report Generation**: Creates multi-sheet Excel reports which include
    - Business performance summaries
    - Trend analysis over time
    - Performance comparisons with change percentages
- **Modern Rust Architecture**: Async processing, error handling, and clean data models
- **Extensible Design**: Ready for frontend integration with Tauri and React

## ️ Technical Aspects

- **Backend**: Rust
- **Data Processing**: Tokio (async), Polars (data analysis)
- **Excel Generation**: rust_xlsxwriter
- **Serialization**: Serde
- **Data Models**: Strongly typed with enums and structs

##  Project Structure

### src/
  - **main.rs** // Application Entry Point & Main Executable
  - **lib.rs** // Library Crate
  - **data_models.rs** // Data Structures & Serialization
  - **data_processing.rs** // Data Extraction & Data Transformation Logic
  - **report_generator.rs** // .xlsx file generation for Excel


## ️ Getting Started

### Prerequisites
- Rust 1.70+
- Cargo

### Installation & Running
```bash
# Clone the repository
git clone 
https://github.com/CMilanoUCA/main-ninten-intern.git
cd main-ninten-intern

# Build and run the project
cargo run
