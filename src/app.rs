use eframe::{egui, epi};
use tnt::{deduction::Deduction, axioms::PEANO, terms::{Variable, Term}};



/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
//#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TNTApp {
    proof: Deduction,
    checks: [bool; 2048],
    variables: Vec<Variable>,
}

impl Default for TNTApp {
    fn default() -> Self {
        Self {
            proof: Deduction::new("Proof", PEANO.clone()),
            checks: [false;2048],
            variables: vec![Variable::new("a"),Variable::new("b"),Variable::new("c"),Variable::new("d"),]
        }
    }
}

impl epi::App for TNTApp {
    fn name(&self) -> &str {
        "Interactive TNT"
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
        let Self { proof, checks, variables} = self;

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

        egui::SidePanel::left("constructor_panel").show(ctx, |ui| {
            ui.heading("Rules of Construction");
            if ui.button("Axiom 1  Aa:~Sa=0").clicked() {
                proof.add_axiom(&PEANO[0]).unwrap();
            }
            if ui.button("Axiom 2  Aa:(a+0)=a").clicked() {
                proof.add_axiom(&PEANO[1]).unwrap();
            }
            if ui.button("Axiom 3  Aa:Ab:(a+Sb)=S(a+b)").clicked() {
                proof.add_axiom(&PEANO[2]).unwrap();
            }
            if ui.button("Axiom 4  Aa:(a*0)=0").clicked() {
                proof.add_axiom(&PEANO[3]).unwrap();
            }
            if ui.button("Axiom 5  Aa:Ab:(a*Sb)=((a*b)+a)").clicked() {
                proof.add_axiom(&PEANO[4]).unwrap();
            }

        });

        egui::SidePanel::right("variable_panel").resizable(false).min_width(150.0).show(ctx, |ui| {
            ui.heading("variables");

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lines of Proof Go Here");
            for theorem in proof.all_theorems_raw() {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut checks[theorem.position], "");
                    ui.label(theorem.formula.to_string());
                });
            }
        });

    }
}
