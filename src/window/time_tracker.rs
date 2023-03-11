use std::time::Duration;

use chrono::Utc;
use eframe::egui::{Align, CentralPanel, Context, Layout, Ui};
use eframe::{App, Frame, Storage};

use crate::data::tracking_day::{LogRecord, LogType, TrackingDay};
use crate::window::widget::CustomWidget;

pub struct TimeTracker {
    pub is_active: bool,
    tracking_day: TrackingDay,
}

impl TimeTracker {
    pub fn new(tracking_day: TrackingDay) -> Self {
        Self {
            is_active: true,
            tracking_day,
        }
    }

    fn render_window(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Today");
            ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                // ui.add(CustomWidget::toggle_switch(&mut self.is_active))
                ui.add(CustomWidget::toggle_switch(self))
            });
        });

        self.render_section_today(ui);
    }

    fn render_section_today(&self, ui: &mut Ui) {
        let working_time = self.tracking_day.get_today_working_time();

        let minutes = (working_time / 60) % 60;
        let hours = (working_time / 60) / 60;

        ui.horizontal(|ui| {
            ui.label("Working Time");
            ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                ui.label(format!("{hours:0>2}:{minutes:0>2}"))
            });
        });
    }

    fn _render_section_week(&self, _ui: &mut Ui) {
        todo!()
    }

    //TODO: add more state events, also put this in some kind of trait
    pub fn on_tracker_state_change(&mut self) {
        self.is_active = !self.is_active;
        if self.is_active {
            self.log_work();
        }
    }

    fn log_work(&mut self) {
        if self.is_active {
            self.tracking_day
                .append_save_record(LogRecord {
                    log_type: LogType::Work,
                    time: Utc::now(),
                    add_seconds: None,
                })
                .expect("Could not save new log record!");
        }
    }
}

impl App for TimeTracker {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.render_window(ui);
        });
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        self.log_work();
    }

    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(30)
    }
}
