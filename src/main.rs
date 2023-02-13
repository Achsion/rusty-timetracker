use eframe::{App, Frame, HardwareAcceleration, NativeOptions, Renderer, run_native, Theme};
use eframe::egui::{CentralPanel, Context, Vec2};

struct TimeTracker {
    working_time: u32
}

impl TimeTracker {
    fn new() -> TimeTracker {
        TimeTracker {
            working_time: 0
        }
    }
}

impl App for TimeTracker {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let minutes = (&self.working_time / 60) % 60;
            let hours = (&self.working_time / 60) / 60;

            ui.label(format!("{:0>2}:{:0>2}", hours, minutes));
            ui.label("Working Time");
        });
    }
}

fn main() {
    let app = TimeTracker::new();

    let window_options = NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Some(Vec2::new(300., 400.)),
        min_window_size: None,
        max_window_size: None,
        resizable: false,
        transparent: false,
        mouse_passthrough: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: HardwareAcceleration::Preferred,
        renderer: Renderer::default(),
        follow_system_theme: cfg!(target_os = "macos") || cfg!(target_os = "windows"),
        default_theme: Theme::Dark,
        run_and_return: true,
        event_loop_builder: None,
        shader_version: None,
        centered: false,
    };

    run_native("TimeTracker", window_options, Box::new(|cc| Box::new(app)))
        .expect("TODO: panic message");
}
