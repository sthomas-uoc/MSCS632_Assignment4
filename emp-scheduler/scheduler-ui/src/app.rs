use std::collections::HashMap;

use scheduler::{Days, Employee, Scheduler, Shifts};

use egui_extras::{Column, TableBuilder};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    name: String,

    #[serde(skip)]
    preferred_shift: Shifts,

    #[serde(skip)]
    employees: Vec<Employee>,

    #[serde(skip)]
    scheduler_run: bool,

    #[serde(skip)]
    schedule: Result<HashMap<Days, HashMap<Shifts, Vec<Employee>>>, String>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            preferred_shift: Shifts::Morning,
            employees: Vec::new(),
            scheduler_run: false,
            schedule: Err("".into())
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Employee Scheduler");

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.horizontal(|ui| {
                ui.label("Shift Preference: ");
                egui::ComboBox::from_id_salt("shift_preference_cb")
                    .selected_text(format!("{}", self.preferred_shift))
                    .show_ui(ui, |ui| {
                       for shift in Shifts::iter() {
                           ui.selectable_value(&mut self.preferred_shift, shift, format!("{}", shift));
                       } 
                    });
                // ui.label(format!("{}", self.preferred_shift));
            });

            if ui.button("Add Employee").clicked() {
                if !self.name.is_empty() {
                    self.employees.push(Employee { name: self.name.clone(), preferred_shift: self.preferred_shift} );
                    self.name = "".into();
                    self.preferred_shift = Shifts::Morning;
                }
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("Employees: {}", &self.employees.len()));
                
                if ui.button("Clear Employees").clicked() {
                    self.employees.clear();
                }

            });


            /*
            ui.scope(|ui| {
                ui.set_max_size([400., 200.].into());

                let mut tb = TableBuilder::new(ui);

                tb = tb.column(Column::auto());
                tb = tb.column(Column::auto());

                tb.header(20., |mut header| {
                    header.col(|ui| {
                       ui.heading("Name");
                    });
                    header.col(|ui| {
                       ui.heading("Preferred Shift");
                    });
                })
                .body(|mut body| {
                    for emp in &self.employees {
                        body.row(30., |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{}", emp.name));
                            });
                            row.col(|ui| {
                                ui.label(format!("{}", emp.preferred_shift));
                            });
                        });
                    }
                });
            });
            */

            ui.separator();
            if ui.button("Get Schedule").clicked() {
                let scheduler = Scheduler { employees: self.employees.clone() };
                self.schedule = scheduler.get_schedule();
                self.scheduler_run = true;
            }

            ui.separator();
            ui.heading("Schedule");

            let status = match &self.schedule {
                Ok(_) => {
                    "Schedule calculated".into()
                },
                Err(error) => {
                    format!("Error getting schedule: {}", error)
                }
            };

            if self.scheduler_run {
                ui.label(status);
            } else {
                ui.label("Scheduler not run");
            }

            if self.schedule.is_ok() {
                ui.scope(|ui| {
                    ui.set_max_size([400., 800.].into());

                    let mut tb = TableBuilder::new(ui);

                    tb = tb.column(Column::auto());
                    for _ in Shifts::iter() {
                        tb = tb.column(Column::auto());
                    }

                    tb.header(20., |mut header| {
                        header.col(|ui| {
                           ui.heading("Day / Shift");
                        });
                        for shift in Shifts::iter() {
                            header.col(|ui| {
                                ui.heading(format!("{}", shift));
                            });
                        }
                    })
                    .body(|mut body| {
                        for day in Days::iter() {
                            body.row(30., |mut row| {
                                row.col(|ui| {
                                    ui.label(format!("{day}"));
                                });
                                for shift in Shifts::iter() {
                                    row.col(|ui| {
                                        if let Ok(res) = &self.schedule {
                                            for emp in &res[&day][&shift] {
                                                ui.label(format!("{}", emp.name));
                                            }
                                        }
                                    });
                                }
                            });
                        }
                    });
                });
            }


            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
