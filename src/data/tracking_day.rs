use chrono::{DateTime, Local};
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::fs::metadata;
use std::path::PathBuf;

pub struct TrackingDay {
    file_path: PathBuf,
    records: Vec<LogRecord>,
}

impl TrackingDay {
    pub fn from_file(file_path: PathBuf) -> Result<Self, csv::Error> {
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

    pub fn clean_records(&mut self) {
        self.records.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        let mut last_log_type = LogType::BreakAdd;
        let mut new_records: Vec<LogRecord> = Vec::new();
        for record in &self.records {
            if record.log_type == LogType::BreakAdd || last_log_type != record.log_type {
                new_records.push(*record);
                last_log_type = record.log_type;
            }
        }

        self.records = new_records;
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct LogRecord {
    time: DateTime<Local>,
    log_type: LogType,
    add_seconds: Option<i64>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
enum LogType {
    Work,
    BreakStart,
    BreakAdd
}
