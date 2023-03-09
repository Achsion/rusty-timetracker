use eframe::egui::{lerp, pos2, vec2, Response, Sense, Ui, Widget, WidgetInfo, WidgetType};
use crate::window::time_tracker::TimeTracker;

pub struct CustomWidget;

impl CustomWidget {
    pub fn toggle_switch(on: &mut TimeTracker) -> impl Widget + '_ {
        move |ui: &mut Ui| CustomWidget::toggle_switch_ui(ui, on)
    }

    fn toggle_switch_ui(ui: &mut Ui, on: &mut TimeTracker) -> Response {
        let desired_size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
        if response.clicked() {
            on.on_tracker_state_change();
        }
        response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, on.is_active, ""));

        if ui.is_rect_visible(rect) {
            let how_on = ui.ctx().animate_bool(response.id, on.is_active);
            let visuals = ui.style().interact_selectable(&response, on.is_active);
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter()
                .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
            let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = pos2(circle_x, rect.center().y);
            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }

        response
    }
}
