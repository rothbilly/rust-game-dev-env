use egui_macroquad::egui::{self, Align2};
pub use engine_macros::IName;
use engine_macros::IVisibility;

use macroquad::{conf, math::clamp, miniquad::window::set_fullscreen};

use crate::{
    data::settings::GLOB_SETTINGS,
    editor::inspector::{Inspector, InspectorWindow},
};

#[derive(Default, IName, IVisibility)]
pub struct SettingsInspector {
    window_name: String,
    is_visible: bool,
    is_value_changed_flag: bool,
}

impl InspectorWindow for SettingsInspector {}

impl Inspector for SettingsInspector {
    fn _setup(&mut self) {
        self.window_name = "settings".to_string();
        self.is_visible = true;
    }

    fn _draw_gui(&mut self) -> bool {
        false
    }

    fn _draw_egui(&mut self, _egui_ctx: &egui_macroquad::egui::Context) {
        let mut settings_lock = GLOB_SETTINGS.write().unwrap();

        egui::Window::new(&self.window_name)
            .anchor(Align2::RIGHT_TOP, [-30., 30.])
            .title_bar(true)
            .open(&mut self.is_visible)
            .show(_egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Egui Render Scale");
                    if ui.button("<").clicked() {
                        settings_lock.egui_render_scale =
                            clamp(settings_lock.egui_render_scale - 0.1, 0.1, 2.0);
                        self.is_value_changed_flag = true
                    }
                    ui.add_enabled(
                        false,
                        egui::DragValue::new(&mut settings_lock.egui_render_scale),
                    );
                    if ui.button(">").clicked() {
                        settings_lock.egui_render_scale =
                            clamp(settings_lock.egui_render_scale + 0.1, 0.1, 2.0);
                        self.is_value_changed_flag = true
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Fullscreen Mode");
                    let _text = if settings_lock.fullscreen {
                        "On"
                    } else {
                        "Off"
                    };

                    if ui
                        .toggle_value(&mut settings_lock.fullscreen, _text)
                        .clicked()
                    {
                        set_fullscreen(settings_lock.fullscreen);
                        self.is_value_changed_flag = true
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("FPS");

                    if ui
                        .selectable_value(&mut settings_lock.swap_interval, 0, "-1")
                        .clicked()
                    {
                        self.is_value_changed_flag = true;
                    }
                    if ui
                        .selectable_value(&mut settings_lock.swap_interval, 1, "60")
                        .clicked()
                    {
                        self.is_value_changed_flag = true;
                    }
                    if ui
                        .selectable_value(&mut settings_lock.swap_interval, 2, "30")
                        .clicked()
                    {
                        self.is_value_changed_flag = true;
                    }
                    if ui
                        .selectable_value(&mut settings_lock.swap_interval, 3, "15")
                        .clicked()
                    {
                        self.is_value_changed_flag = true;
                    }
                });

                if self.is_value_changed_flag {
                    if ui.button("Save Settings").clicked() {
                        if let Ok(_) = settings_lock.save() {
                            self.is_value_changed_flag = false
                        }
                    }
                }
            });
    }
}
