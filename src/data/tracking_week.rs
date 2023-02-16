use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::fs::metadata;

pub struct TrackingWeek {
    file_path: String,
    records: Vec<DummyRecord>,
}

impl TrackingWeek {
    pub fn from_file(file_path: String) -> Result<Self, csv::Error> {
        if metadata(&file_path).is_err() {
            return Ok(Self {
                file_path,
                records: Vec::new(),
            });
        }

        let mut reader = Reader::from_path(&file_path)?;

        Ok(Self {
            file_path,
            records: Vec::from_iter(reader.deserialize().filter_map(|r| r.ok())),
        })
    }

    pub fn save(&self) -> Result<(), csv::Error> {
        let mut writer = Writer::from_path(self.file_path.clone())?;

        for record in &self.records {
            writer.serialize(record)?;
        }

        writer.flush()?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct DummyRecord {
    name: String,
    positive_number: u64,
    negative_number: i64,
    float_number: f64,
    description: Option<String>,
}
