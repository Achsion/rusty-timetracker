use crate::window::widget::CustomWidget;
use eframe::egui::{Align, CentralPanel, Context, Layout, Ui};
use eframe::{App, Frame};

pub struct TimeTracker {
    is_active: bool,
    working_time: u32,
}

impl TimeTracker {
    pub fn new() -> Self {
        Self {
            is_active: true,
            working_time: 27967,
        }
    }

    fn render_window(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Today");
            ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                ui.add(CustomWidget::toggle_switch(&mut self.is_active))
            });
        });

        self.render_section_today(ui);
    }

    fn render_section_today(&self, ui: &mut Ui) {
        let minutes = (&self.working_time / 60) % 60;
        let hours = (&self.working_time / 60) / 60;

        ui.horizontal(|ui| {
            ui.label("Working Time");
            ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                ui.label(format!("{:0>2}:{:0>2}", hours, minutes))
            });
        });
    }

    fn _render_section_week(&self, _ui: &mut Ui) {}
}

impl App for TimeTracker {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.render_window(ui);
        });
    }
}
