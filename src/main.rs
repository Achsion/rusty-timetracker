use std::error::Error;
use std::ops::Add;
use std::process;

use chrono::{Datelike, Duration, Utc};
use eframe::egui::{
    Context, FontData, FontDefinitions, FontFamily, FontId, IconData, TextStyle, ViewportBuilder,
};
use eframe::{run_native, NativeOptions};

use crate::data::day_log::{DayLog, LogRecord, LogType};
use crate::manager::directory_manager::DirectoryType;
use crate::window::time_tracker::TimeTracker;

mod data {
    pub mod day_log;
}
mod manager {
    pub mod directory_manager;
}
mod window {
    pub mod time_tracker;
    pub mod widget;
}

fn main() {
    if let Err(err) = setup() {
        println!("{}", err);
        process::exit(1);
    }
}

fn setup() -> Result<(), Box<dyn Error>> {
    let _config_dir_path =
        DirectoryType::Config.setup_directory("de", "Achsion", "RustyTimeTracker")?;
    let data_dir_path = DirectoryType::Data.setup_directory("de", "Achsion", "RustyTimeTracker")?;

    let window_options = setup_custom_options();

    let mut tracking_day = DayLog::from_file(data_dir_path.join("day_log.csv"))?;

    tracking_day.clean_records();

    let last_log_optional = tracking_day.last_log(vec![LogType::Work, LogType::Break]);
    if let Some(last_log) = last_log_optional {
        let now = Utc::now();
        if last_log.log_type == LogType::Work
            && last_log.time.day() == now.day()
            && now
                .signed_duration_since(last_log.time)
                .gt(&Duration::minutes(30))
        {
            tracking_day.append_record(LogRecord {
                time: last_log.time.add(Duration::minutes(1)),
                log_type: LogType::Break,
                add_seconds: None,
            });
        }
    }

    tracking_day.save_records()?;

    let mut time_tracker = TimeTracker::new(tracking_day);
    if time_tracker.is_active {
        time_tracker.log_work_now();
    }

    time_tracker.update_working_time();

    run_native(
        "TimeTracker",
        window_options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(time_tracker)
        }),
    )?;

    Ok(())
}

fn setup_custom_options() -> NativeOptions {
    NativeOptions {
        viewport: ViewportBuilder::default()
            .with_icon(load_icon())
            .with_inner_size([300., 400.])
            .with_resizable(false),
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
