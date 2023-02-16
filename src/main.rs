mod data {
    pub mod tracking_week;
}
mod window {
    pub mod time_tracker;
    pub mod widget;
}

use crate::data::tracking_week::TrackingWeek;
use crate::window::time_tracker::TimeTracker;
use directories::ProjectDirs;
use eframe::egui::{Context, FontData, FontDefinitions, FontFamily, FontId, TextStyle, Vec2};
use eframe::{run_native, IconData, NativeOptions};
use std::env::current_dir;
use std::error::Error;
use std::fs::{create_dir_all, metadata};
use std::path::PathBuf;
use std::{io, process};

fn main() {
    if let Err(err) = setup() {
        println!("{}", err);
        process::exit(1);
    }
}

fn setup() -> Result<(), Box<dyn Error>> {
    let config_dir_path = setup_config_directory()?;

    let read_result = TrackingWeek::from_file(config_dir_path.join("test.csv"))?;
    read_result.save()?;

    let window_options = setup_custom_options();

    run_native(
        "TimeTracker",
        window_options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(TimeTracker::new())
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

fn setup_config_directory() -> Result<PathBuf, io::Error> {
    let config_path = if cfg!(debug_assertions) {
        let mut working_dir = current_dir()?;
        working_dir.push("timetracker-tmp");
        working_dir
    } else {
        ProjectDirs::from("de", "Achsion", "TimeTracker")
            .unwrap()
            .config_dir()
            .to_path_buf()
    };

    if metadata(&config_path).is_err() {
        create_dir_all(&config_path)?;
    }

    Ok(config_path)
}
