use anyhow::Result;
use polars::prelude::*;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let data = reqwest::get(url).await?.text().await?;

    let df = CsvReader::new(Cursor::new(data)).infer_schema(Some(16)).finish()?;

    let n: i32 = 500;
    let filtered = df.filter(&df["new_cases"].gt(n))?;
    println!(
        "{:?}",
        filtered.select([
            "location",
            "total_cases",
            "new_cases",
            "total_deaths",
            "total_cases_per_million"
        ])
    );
    Ok(())
}