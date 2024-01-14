use std::time::Duration;

use chrono::Utc;
use eframe::egui::{Align, CentralPanel, Context, Layout, Ui};
use eframe::{App, Frame, Storage};

use crate::data::day_log::{DayLog, LogRecord, LogType};
use crate::window::widget::{CustomWidget, WithToggleSwitch};

pub struct TimeTracker {
    pub is_active: bool,
    pub tracking_day: DayLog,
    today_working_time: i64,
    week_working_time: i64,
}

impl TimeTracker {
    pub fn new(tracking_day: DayLog) -> Self {
        let today_working_time = tracking_day.get_today_working_seconds_sum();
        let week_working_time = tracking_day.tmp_get_week_working_seconds_without_today_sum();

        Self {
            is_active: true,
            tracking_day,
            today_working_time,
            week_working_time,
        }
    }

    fn render_window(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Today");
            ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                ui.add(CustomWidget::toggle_switch(self))
            });
        });

        self.render_section_today(ui);
        self.render_section_week(ui);
    }

    fn render_section_today(&self, ui: &mut Ui) {
        let minutes = (self.today_working_time / 60) % 60;
        let hours = (self.today_working_time / 60) / 60;

        ui.horizontal(|ui| {
            ui.label("Working Time");
            ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                ui.label(format!("{hours:0>2}:{minutes:0>2}"))
            });
        });
    }

    fn render_section_week(&self, ui: &mut Ui) {
        let week_working_time_sum = self.today_working_time + self.week_working_time;
        let minutes = (week_working_time_sum / 60) % 60;
        let hours = (week_working_time_sum / 60) / 60;

        ui.horizontal(|ui| {
            ui.label("Weekly Working Time");
            ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                ui.label(format!("{hours:0>2}:{minutes:0>2}"))
            });
        });
    }

    pub fn log_work_now(&mut self) {
        self.tracking_day
            .append_save_record(LogRecord {
                log_type: LogType::Work,
                time: Utc::now(),
                add_seconds: None,
            })
            .expect("Could not save new log record!");

        self.update_working_time();
    }

    fn log_break_start(&mut self) {
        self.tracking_day
            .append_save_record(LogRecord {
                log_type: LogType::Break,
                time: Utc::now(),
                add_seconds: None,
            })
            .expect("Could not save new log record!");
    }

    pub fn update_working_time(&mut self) {
        self.today_working_time = self.tracking_day.get_today_working_seconds_sum();
    }
}

impl App for TimeTracker {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.render_window(ui);
        });
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        if self.is_active {
            self.log_work_now();
        }
    }

    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(30)
    }
}

impl WithToggleSwitch for TimeTracker {
    fn on_tracker_state_change(&mut self) {
        self.is_active = !self.is_active;
        if self.is_active {
            self.log_work_now();
        } else {
            self.log_break_start();
        }
    }

    fn get_toggle_state(&self) -> bool {
        self.is_active
    }
}
