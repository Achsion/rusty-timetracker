use eframe::egui::{CentralPanel, Context, Ui};
use eframe::{App, Frame};

pub struct TimeTracker {
    working_time: u32,
}

impl TimeTracker {
    pub fn new() -> Self {
        Self {
            working_time: 45367,
        }
    }

    fn render_times(&self, ui: &mut Ui) {
        let minutes = (&self.working_time / 60) % 60;
        let hours = (&self.working_time / 60) / 60;

        ui.label(format!("{:0>2}:{:0>2}", hours, minutes));
        ui.label("Working Time");
    }
}

impl App for TimeTracker {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.render_times(ui);
        });
    }
}
