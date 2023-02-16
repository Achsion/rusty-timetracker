use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

pub struct TrackingWeek {
    file_path: String,
}

impl TrackingWeek {
    pub fn from_file(file: &str) -> Result<Self, csv::Error> {
        //TODO: get full path from attribute
        let file_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), file);
        let mut reader = Reader::from_path(file_path.clone())?;

        //TODO: save result records in TrackingWeek
        for result in reader.deserialize() {
            let record: DummyRecord = result?;
            println!("{:?}", record);
        }

        Ok(Self { file_path })
    }

    pub fn save(&self) -> Result<(), csv::Error> {
        let mut writer = Writer::from_path(self.file_path.clone())?;

        //TODO: saved all records in file
        writer.serialize(DummyRecord {
            name: String::from("Amogus"),
            positive_number: 372,
            negative_number: -5678,
            float_number: 9.789,
            description: Some(String::from("Sussy baka :3")),
        })?;

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
