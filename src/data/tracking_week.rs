use csv::Reader;
use serde::Deserialize;

pub struct TrackingWeek;

impl TrackingWeek {
    pub fn from_file(path: &str) -> Result<Self, csv::Error> {
        let mut reader = Reader::from_path(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path))?;

        for result in reader.deserialize() {
            let record: DummyRecord = result?;
            println!("{:?}", record);
        }

        Ok(Self)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DummyRecord {
    name: String,
    positive_number: u64,
    negative_number: i64,
    float_number: f64,
    description: Option<String>
}
