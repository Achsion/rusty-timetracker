use chrono::{DateTime, Utc};
use csv::{Reader, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{metadata, OpenOptions};
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

    pub fn save_records(&self) -> Result<(), csv::Error> {
        let mut writer = Writer::from_path(&self.file_path)?;

        for record in &self.records {
            writer.serialize(record)?;
        }

        writer.flush()?;
        Ok(())
    }

    pub fn clean_records(&mut self) {
        self.records
            .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        let mut last_log_type = LogType::Unknown;
        let mut new_records: Vec<LogRecord> = Vec::new();
        for record in &self.records {
            if record.log_type == LogType::BreakAdd || last_log_type != record.log_type {
                new_records.push(*record);
                last_log_type = record.log_type;
            }
        }

        self.records = new_records;
    }

    pub fn append_save_record(&mut self, log_record: LogRecord) -> Result<(), Box<dyn Error>> {
        let should_write_headers = self.records.is_empty();
        self.append_record(log_record);

        let file_writer = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        let mut writer = WriterBuilder::new()
            .has_headers(should_write_headers)
            .from_writer(file_writer);
        writer.serialize(log_record)?;
        writer.flush()?;

        Ok(())
    }

    pub fn append_record(&mut self, log_record: LogRecord) {
        self.records.push(log_record);
    }

    pub fn get_today_working_time(&self) -> u32 {
        27967
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LogRecord {
    pub time: DateTime<Utc>,
    pub log_type: LogType,
    pub add_seconds: Option<i64>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum LogType {
    Work,
    BreakStart,
    BreakAdd,
    Unknown,
}
