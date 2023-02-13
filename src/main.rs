use eframe::{App, Frame, HardwareAcceleration, NativeOptions, Renderer, run_native, Theme};
use eframe::egui::{CentralPanel, Context, FontData, FontDefinitions, FontFamily, FontId, TextStyle, Vec2};

struct TimeTracker {
    working_time: u32
}

impl TimeTracker {
    fn new() -> Self {
        Self {
            working_time: 0,
        }
    }
}

impl App for TimeTracker {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let minutes = (&self.working_time / 60) % 60;
            let hours = (&self.working_time / 60) / 60;

            ui.label(format!("{:0>2}:{:0>2}", hours, minutes));
            ui.label("Working Time");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let window_options = setup_custom_options();

    run_native(
        "TimeTracker",
        window_options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(TimeTracker::new())
        })
    )
}

fn setup_custom_options() -> NativeOptions {
    NativeOptions {
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
    }
}

fn setup_custom_fonts(ctx: &Context) {
    let mut font_def = FontDefinitions::default();

    font_def.font_data.insert(
        "Lato".to_owned(),
        FontData::from_static(include_bytes!(
            "../resources/fonts/Lato-Regular.ttf"
        )),
    );

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(30.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(18.0, FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Small, FontId::new(10.0, FontFamily::Proportional)),
    ].into();
    ctx.set_style(style);

    // Put my font first (highest priority) for proportional text
    font_def
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "Lato".to_owned());
    // Put my font as last fallback for monospace
    font_def
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("Lato".to_owned());

    ctx.set_fonts(font_def);
}
