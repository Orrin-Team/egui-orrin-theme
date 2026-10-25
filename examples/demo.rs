//! `cargo run --example demo` — every surface the theme styles, and a switcher
//! for the themes in `themes/`.

use eframe::egui;
use egui_orrin_theme::{self as theme, ThemeSet, icons};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "egui-orrin-theme",
        options,
        Box::new(|cc| {
            theme::install_fonts(&cc.egui_ctx);
            let (themes, errors) =
                ThemeSet::load(concat!(env!("CARGO_MANIFEST_DIR"), "/themes").as_ref());
            for error in errors {
                eprintln!("{error}");
            }
            theme::apply(&cc.egui_ctx, themes.active());
            Ok(Box::new(Demo {
                themes,
                selected: 0,
                name: "Sponza".to_owned(),
                intensity: 3.2,
                cast_shadows: true,
            }))
        }),
    )
}

struct Demo {
    themes: ThemeSet,
    selected: usize,
    name: String,
    intensity: f32,
    cast_shadows: bool,
}

const ROWS: [(&str, icons::Icon, egui::Color32); 6] = [
    (
        "Camera",
        icons::POINTER,
        egui::Color32::from_rgb(0x6C, 0xC4, 0xB8),
    ),
    ("Sun", icons::SUN, egui::Color32::from_rgb(0xE2, 0xB6, 0x55)),
    (
        "Floor",
        icons::BOX,
        egui::Color32::from_rgb(0x7F, 0xA7, 0xE8),
    ),
    (
        "Columns",
        icons::LAYERS,
        egui::Color32::from_rgb(0x7F, 0xA7, 0xE8),
    ),
    (
        "Lamp",
        icons::LIGHTBULB,
        egui::Color32::from_rgb(0xE2, 0xB6, 0x55),
    ),
    (
        "Orbit.cs",
        icons::FILE_CODE,
        egui::Color32::from_rgb(0xB1, 0x93, 0xF0),
    ),
];

impl eframe::App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let active = self.themes.active().clone();

        egui::TopBottomPanel::top("menu")
            .frame(egui::Frame::new().fill(active.bar()).inner_margin(4))
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        let _ = ui.button(icons::with_label(icons::PLUS, None, "New scene"));
                        let _ = ui.button(icons::with_label(icons::REFRESH, None, "Reload"));
                    });
                    ui.menu_button("Theme", |ui| {
                        let names: Vec<String> = self.themes.names().map(str::to_owned).collect();
                        for name in names {
                            let current = name == active.name;
                            if ui.selectable_label(current, &name).clicked()
                                && self.themes.select(&name)
                            {
                                theme::apply(ctx, self.themes.active());
                                ui.close_menu();
                            }
                        }
                    });
                    ui.separator();
                    icons::button(ui, icons::PLAY, icons::ROW, theme::OK, false);
                    icons::button(ui, icons::PAUSE, icons::ROW, active.accent(), false);
                });
            });

        egui::TopBottomPanel::bottom("status")
            .frame(egui::Frame::new().fill(active.bar()).inner_margin(4))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(icons::text(icons::CHECK, icons::ROW - 2.0, theme::OK));
                    ui.colored_label(theme::OK, "Built");
                    ui.separator();
                    ui.colored_label(theme::PENDING, "3 warnings");
                    ui.separator();
                    ui.monospace("16.61 ms  60 fps");
                });
            });

        egui::SidePanel::left("outliner")
            .default_width(220.0)
            .show(ctx, |ui| {
                ui.heading("Scene");
                ui.add_space(4.0);
                for (index, (name, icon, tint)) in ROWS.iter().enumerate() {
                    let row = ui.horizontal(|ui| {
                        ui.label(icons::text(*icon, icons::ROW - 2.0, *tint));
                        ui.selectable_label(self.selected == index, *name)
                    });
                    if index % 2 == 1 {
                        ui.painter()
                            .rect_filled(row.response.rect, 0.0, theme::ROW_BAND);
                    }
                    if row.inner.clicked() {
                        self.selected = index;
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(ROWS[self.selected].0);
            ui.add_space(4.0);
            egui::CollapsingHeader::new("Light")
                .default_open(true)
                .show(ui, |ui| {
                    egui::Grid::new("light").num_columns(2).show(ui, |ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut self.name);
                        ui.end_row();
                        ui.label("Intensity (lux)");
                        ui.add(egui::Slider::new(&mut self.intensity, 0.0..=10.0));
                        ui.end_row();
                        ui.label("Shadows");
                        ui.checkbox(&mut self.cast_shadows, "Cast");
                        ui.end_row();
                    });
                });
            egui::CollapsingHeader::new("Console").show(ui, |ui| {
                ui.monospace("[info] loaded sponza.gltf in 182 ms");
                ui.colored_label(theme::PENDING, "[warn] KHR_texture_transform ignored");
                ui.colored_label(theme::ERROR, "[error] Orbit.cs: CS0103");
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let _ = ui.button("Apply");
                let _ = ui.button(icons::with_label(
                    icons::TRASH,
                    Some(theme::ERROR),
                    "Delete",
                ));
            });
        });
    }
}
