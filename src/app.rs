use eframe::{egui, epi};
use num::{BigUint, Integer};




/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
//#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    a: u32,
    b: u32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            a: 1,
            b: 1,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { a, b} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
            egui::warn_if_debug_build(ui);
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Panel");

            // Control a and b
            ui.horizontal(|ui| {
                ui.label("a = ");
                ui.add(egui::DragValue::new(a).speed(1.0));
                if ui.small_button("-").clicked() {
                    *a = a.saturating_sub(1);
                }
                if ui.small_button("+").clicked() {
                    *a = a.saturating_add(1);
                }
            });

            ui.horizontal(|ui| {
                ui.label("b = ");
                ui.add(egui::DragValue::new(b).speed(1.0));
                if ui.small_button("-").clicked() {
                    *b = b.saturating_sub(1);
                }
                if ui.small_button("+").clicked() {
                    *b = b.saturating_add(1);
                }
            });

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            let big_a = BigUint::from(*a);
            let big_b = BigUint::from(*b);

            let (gcd, lcm) = big_a.gcd_lcm(&big_b);
            let pow = big_a.pow(*b);
            let (quo, rem) = big_a.div_rem(&big_b);

            ui.horizontal(|ui| {
                ui.label("gcd(a,b) =");
                ui.label(gcd.to_string());
            });

            ui.horizontal(|ui| {
                ui.label("lcm(a,b) =");
                ui.label(lcm.to_string());
            });

            ui.horizontal(|ui| {
                ui.label("a^b =");
                ui.label(pow.to_string());
            });

            ui.horizontal(|ui| {
                ui.label("a / b =");
                ui.label(format!("{} r{}", quo, rem));
            });

        });

    }
}
