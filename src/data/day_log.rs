use chrono::{DateTime, Datelike, Utc};
use csv::{Reader, Writer, WriterBuilder};
use grouping_by::GroupingBy;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{metadata, OpenOptions};
use std::path::PathBuf;

pub struct DayLog {
    file_path: PathBuf,
    records: Vec<LogRecord>,
}

impl DayLog {
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

        let mut new_records: Vec<LogRecord> = Vec::new();

        //TODO: sort out records older than today and persist them to the overall working stats
        self.records
            .iter()
            .grouping_by(|r| r.time.day())
            .iter()
            .for_each(|(_, daily_records)| {
                let mut last_log_type = LogType::Unknown;

                for i in 0..daily_records.len() {
                    let record = daily_records.get(i).unwrap();

                    if i == daily_records.len() - 1
                        || record.log_type == LogType::BreakAdd
                        || last_log_type != record.log_type
                    {
                        new_records.push(**record);
                        last_log_type = record.log_type;
                    }
                }
            });

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

    fn calculate_work_seconds_diff(
        &self,
        last_time_opt: Option<DateTime<Utc>>,
        newer_time: DateTime<Utc>,
    ) -> i64 {
        if let Some(last_time) = last_time_opt {
            return newer_time.signed_duration_since(last_time).num_seconds();
        }

        0
    }

    pub fn get_today_working_seconds_sum(&self) -> i64 {
        let today = Utc::now().num_days_from_ce();
        let mut working_time_sum: i64 = 0;
        let mut last_work_log_time: Option<DateTime<Utc>> = None;

        self.records
            .iter()
            .filter(|r| r.time.num_days_from_ce() == today)
            .for_each(|record| {
                match record.log_type {
                    LogType::BreakAdd => {
                        if let Some(add_seconds) = record.add_seconds {
                            working_time_sum -= add_seconds;
                        }
                    }
                    LogType::Break => {
                        working_time_sum +=
                            self.calculate_work_seconds_diff(last_work_log_time, record.time);
                        last_work_log_time = None;
                    }
                    LogType::Work => {
                        working_time_sum +=
                            self.calculate_work_seconds_diff(last_work_log_time, record.time);
                        last_work_log_time = Some(record.time);
                    }
                    _ => {}
                };
            });

        working_time_sum += self.calculate_work_seconds_diff(last_work_log_time, Utc::now());

        working_time_sum
    }

    pub fn tmp_get_week_working_seconds_without_today_sum(&self) -> i64 {
        // TODO: this is only a temporary solution to display the weekly working time
        //       this exists solely because i am too lazy rn to implement a proper week_log data type thingy but i still want to see the time

        let current_week = Utc::now().iso_week();
        let mut working_time_sum: i64 = 0;
        let mut last_work_log_time: Option<DateTime<Utc>> = None;

        self.records
            .iter()
            .filter(|r| r.time.iso_week().eq(&current_week)) //TODO: divide/map per day and calculate accordingly
            .for_each(|record| {
                //TODO: i know that this IS a duplicate code BUT hear me out: this will be removed after the data change so i dont really care
                match record.log_type {
                    LogType::BreakAdd => {
                        if let Some(add_seconds) = record.add_seconds {
                            working_time_sum -= add_seconds;
                        }
                    }
                    LogType::Break => {
                        working_time_sum +=
                            self.calculate_work_seconds_diff(last_work_log_time, record.time);
                        last_work_log_time = None;
                    }
                    LogType::Work => {
                        working_time_sum +=
                            self.calculate_work_seconds_diff(last_work_log_time, record.time);
                        last_work_log_time = Some(record.time);
                    }
                    _ => {}
                };
            });

        working_time_sum += self.calculate_work_seconds_diff(last_work_log_time, Utc::now());

        working_time_sum
    }

    pub fn last_log(&self, type_filter: Vec<LogType>) -> Option<&LogRecord> {
        self.records
            .iter()
            .filter(|r| type_filter.is_empty() || type_filter.contains(&r.log_type))
            .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap())
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
    Break,
    BreakAdd,
    Unknown,
}
