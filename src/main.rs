mod data {
    pub mod tracking_day;
}
mod manager {
    pub mod directory_manager;
}
mod window {
    pub mod time_tracker;
    pub mod widget;
}

use crate::data::tracking_day::TrackingDay;
use crate::manager::directory_manager::DirectoryType;
use crate::window::time_tracker::TimeTracker;
use eframe::egui::{Context, FontData, FontDefinitions, FontFamily, FontId, TextStyle, Vec2};
use eframe::{run_native, IconData, NativeOptions};
use std::error::Error;
use std::process;

fn main() {
    if let Err(err) = setup() {
        println!("{}", err);
        process::exit(1);
    }
}

fn setup() -> Result<(), Box<dyn Error>> {
    let _config_dir_path = DirectoryType::Config.setup_directory("de", "Achsion", "TimeTracker")?;
    let data_dir_path = DirectoryType::Data.setup_directory("de", "Achsion", "TimeTracker")?;

    let window_options = setup_custom_options();

    let mut tracking_day = TrackingDay::from_file(data_dir_path.join("test.csv"))?;
    tracking_day.clean_records();
    tracking_day.save_records()?;

    run_native(
        "TimeTracker",
        window_options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(TimeTracker::new(tracking_day))
        }),
    )?;

    Ok(())
}

fn setup_custom_options() -> NativeOptions {
    NativeOptions {
        icon_data: Some(load_icon()),
        initial_window_size: Some(Vec2::new(300., 400.)),
        resizable: false,
        ..Default::default()
    }
}

fn setup_custom_fonts(ctx: &Context) {
    let mut font_def = FontDefinitions::default();

    font_def.font_data.insert(
        "Lato".to_owned(),
        FontData::from_static(include_bytes!("../resources/fonts/Lato-Regular.ttf")),
    );

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(30.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(18.0, FontFamily::Proportional)),
        (
            TextStyle::Monospace,
            FontId::new(14.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Button,
            FontId::new(14.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(10.0, FontFamily::Proportional),
        ),
    ]
    .into();
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

fn load_icon() -> IconData {
    let image_bytes = include_bytes!("../resources/icons/icon.png");

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(image_bytes)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
