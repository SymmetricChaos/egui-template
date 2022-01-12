use eframe::{egui, epi};

pub fn gcd_fn(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
//#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    a: u64,
    b: u64,
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
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Panel");

            ui.horizontal(|ui| {
                ui.label("a = ");
                ui.add(egui::DragValue::new(a).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("b = ");
                ui.add(egui::DragValue::new(b).speed(1.0));
            });

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.horizontal(|ui| {
                ui.label("gcd(a,b) = ");
                ui.label(gcd_fn(*a,*b).to_string());
            });

            ui.horizontal(|ui| {
                ui.label("a + b = ");
                ui.label((*a+*b).to_string());
            });

            ui.horizontal(|ui| {
                ui.label("a * b = ");
                ui.label((*a**b).to_string());
            });
            egui::warn_if_debug_build(ui);
        });

    }
}
