extern crate eframe;
extern crate sysinfo;

use eframe::{Frame, egui};
use egui::{pos2, vec2, Color32, RichText, ViewportBuilder};
use std::time::{Duration, Instant};
use sysinfo::System;

struct OsStat {
    ram_total_mib: f64,
    ram_usage_mib: f64,
    ram_usage_percent: f64,
    cpu_usage_percent: f32,
    last_update: Instant,
    system: System,
}

impl Default for OsStat {
    fn default() -> Self {
        OsStat {
            ram_total_mib: 0.0,
            ram_usage_mib: 0.0,
            ram_usage_percent: 0.0,
            cpu_usage_percent: 0.0,
            last_update: Instant::now() - Duration::from_secs(1),
            system: System::new_all(),
        }
    }
}

impl eframe::App for OsStat {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if self.last_update.elapsed() >= Duration::from_secs(1) {
            self.system.refresh_cpu_usage();
            self.system.refresh_memory();

            self.ram_total_mib = (self.system.total_memory() as f64) / 1024.0 / 1024.0;
            self.ram_usage_mib = (self.system.used_memory() as f64) / 1024.0 / 1024.0;
            self.ram_usage_percent = (self.ram_usage_mib / self.ram_total_mib) * 100.0;
            self.cpu_usage_percent = self.system.global_cpu_usage();

            self.last_update = Instant::now();
        }

        let ram_color = {
            if self.ram_usage_percent < 10.0 {
                Color32::GREEN
            } else if self.ram_usage_percent > 90.00 {
                Color32::RED
            } else {
                Color32::ORANGE
            }
        };

        let cpu_color = {
            if self.cpu_usage_percent < 10.0 {
                Color32::GREEN
            } else if self.cpu_usage_percent > 90.00 {
                Color32::RED
            } else {
                Color32::ORANGE
            }
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("OSStats");
                ui.label(
                    RichText::new(format!("RAM: {:.2}%", self.ram_usage_percent)).color(ram_color),
                );
                ui.label(format!(
                    "{:.2}/{:.2} MiB",
                    self.ram_usage_mib, self.ram_total_mib,
                ));
                ui.label(
                    RichText::new(format!("CPU: {:.2}%", self.cpu_usage_percent)).color(cpu_color),
                );
            });
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("OsStat")
            .with_resizable(true)
            .with_decorations(false)
            .with_position(pos2(0.0, 0.0))
            .with_transparent(true)
            .with_inner_size(vec2(200.0, 100.0)),
        
        ..Default::default()
    };

    let _ = eframe::run_native(
        "OsStat",
        options,
        Box::new(|_cc| Ok(Box::<OsStat>::default())),
    );
}
