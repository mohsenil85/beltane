//! Beltane GUI — egui/eframe frontend.
//!
//! Two-panel window: transport controls (hex-arch integration) + animated waveform.
//! Run: `cargo run -p beltane-gui`

use std::sync::{Arc, Mutex};
use std::time::Instant;

use eframe::egui;

use beltane_ports::{AudioPort, Command, ProjectionPort, TransportView};
use beltane_runtime::Runtime;

// ---------------------------------------------------------------------------
// Fake adapters (placeholder until real audio adapter exists)
// ---------------------------------------------------------------------------

struct FakeAudio;
impl AudioPort for FakeAudio {
    fn sync_transport(&mut self, _is_playing: bool) {}
}

struct SharedProjection {
    view: Arc<Mutex<TransportView>>,
}

impl ProjectionPort for SharedProjection {
    fn publish_transport(&mut self, view: TransportView) {
        *self.view.lock().unwrap() = view;
    }
}

// ---------------------------------------------------------------------------
// Application
// ---------------------------------------------------------------------------

struct App {
    runtime: Runtime<FakeAudio, SharedProjection>,
    shared_view: Arc<Mutex<TransportView>>,
    is_playing: bool,
    last_key: String,
    phase: f32,
    start: Instant,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let shared_view = Arc::new(Mutex::new(TransportView { is_playing: false }));
        let projection = SharedProjection {
            view: Arc::clone(&shared_view),
        };
        let runtime = Runtime::with_defaults(FakeAudio, projection);

        Self {
            runtime,
            shared_view,
            is_playing: false,
            last_key: String::from("(none)"),
            phase: 0.0,
            start: Instant::now(),
        }
    }

    fn toggle_play(&mut self) {
        self.runtime.dispatch(Command::TogglePlay);
        self.is_playing = self.shared_view.lock().unwrap().is_playing;
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Keyboard handling
        ctx.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Key {
                        key: egui::Key::Space,
                        pressed: true,
                        repeat: false,
                        ..
                    } => {
                        self.toggle_play();
                    }
                    egui::Event::Key {
                        key,
                        pressed: true,
                        repeat: false,
                        modifiers,
                        ..
                    } => {
                        let mut parts = Vec::new();
                        if modifiers.ctrl { parts.push("Ctrl".to_string()); }
                        if modifiers.shift { parts.push("Shift".to_string()); }
                        if modifiers.alt { parts.push("Alt".to_string()); }
                        if modifiers.command { parts.push("Cmd".to_string()); }
                        parts.push(format!("{key:?}"));
                        self.last_key = parts.join("+");
                    }
                    _ => {}
                }
            }
        });

        // Update animation phase
        if self.is_playing {
            self.phase = (Instant::now() - self.start).as_secs_f32() * 2.0;
            ctx.request_repaint();
        }

        // Transport panel
        egui::TopBottomPanel::top("transport").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let status = if self.is_playing { "Playing" } else { "Stopped" };
                ui.heading(format!("Transport: {status}"));

                let toggle_label = if self.is_playing { "Stop" } else { "Play" };
                if ui.button(toggle_label).clicked() {
                    self.toggle_play();
                }

                ui.separator();
                ui.label(format!("Last key: {}", self.last_key));
            });
        });

        // Waveform panel
        egui::CentralPanel::default().show(ctx, |ui| {
            let available = ui.available_size();
            let (response, painter) =
                ui.allocate_painter(available, egui::Sense::hover());
            let rect = response.rect;
            let w = rect.width();
            let h = rect.height();
            let mid_y = rect.top() + h / 2.0;

            // Dark background
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(20, 20, 30));

            // Horizontal zero-crossing grid line
            painter.line_segment(
                [
                    egui::pos2(rect.left(), mid_y),
                    egui::pos2(rect.right(), mid_y),
                ],
                egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(77, 77, 102, 153)),
            );

            // Waveform polyline: sin(t) + 0.3*sin(3t)
            let num_points = w as usize;
            let points: Vec<egui::Pos2> = (0..=num_points)
                .map(|i| {
                    let x = rect.left() + i as f32;
                    let t = (i as f32 / w) * 4.0 * std::f32::consts::PI + self.phase;
                    let y = (t.sin() + 0.3 * (3.0 * t).sin()) * (h * 0.35);
                    egui::pos2(x, mid_y - y)
                })
                .collect();

            // Glow layer
            painter.add(egui::Shape::line(
                points.clone(),
                egui::Stroke::new(6.0, egui::Color32::from_rgba_premultiplied(51, 153, 255, 64)),
            ));

            // Main waveform line
            painter.add(egui::Shape::line(
                points,
                egui::Stroke::new(2.0, egui::Color32::from_rgba_premultiplied(77, 179, 255, 230)),
            ));

            // Playhead
            let playhead_x = if self.is_playing {
                rect.left() + (self.phase * 40.0) % w
            } else {
                rect.left()
            };
            painter.line_segment(
                [
                    egui::pos2(playhead_x, rect.top()),
                    egui::pos2(playhead_x, rect.bottom()),
                ],
                egui::Stroke::new(2.0, egui::Color32::from_rgba_premultiplied(255, 77, 77, 204)),
            );
        });
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Beltane",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
