//! Graph view of Reservoir message nodes, compiled to wasm and served by `reservoir admin`.

mod graph;

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use eframe::egui::{self, Color32, Sense, Stroke, Vec2};
use wasm_bindgen::prelude::*;

use graph::{Graph, GraphData};

#[wasm_bindgen]
pub struct Admin {
    runner: eframe::WebRunner,
}

#[wasm_bindgen]
impl Admin {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    /// Call once from JavaScript with the canvas to draw on.
    pub async fn start(&self, canvas: web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
        self.runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(App::new(cc)))),
            )
            .await
    }
}

impl Default for Admin {
    fn default() -> Self {
        Self::new()
    }
}

type Loaded = Arc<Mutex<Option<Result<GraphData, String>>>>;

struct App {
    loaded: Loaded,
    status: String,
    data: Option<GraphData>,
    graph: Option<Graph>,
    partitions: Vec<String>,
    partition: Option<String>,
    show_traces: bool,
    selected: Option<usize>,
    zoom: f32,
    pan: Vec2,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            loaded: Arc::default(),
            status: "Loading…".into(),
            data: None,
            graph: None,
            partitions: vec![],
            partition: None,
            show_traces: true,
            selected: None,
            zoom: 1.0,
            pan: Vec2::ZERO,
        };
        app.fetch(&cc.egui_ctx);
        app
    }

    fn fetch(&mut self, ctx: &egui::Context) {
        self.status = "Loading…".into();
        let slot = self.loaded.clone();
        let ctx = ctx.clone();
        ehttp::fetch(ehttp::Request::get("api/graph"), move |res| {
            let parsed = res
                .map_err(|e| e.to_string())
                .and_then(|r| {
                    if r.ok {
                        serde_json::from_slice::<GraphData>(&r.bytes).map_err(|e| e.to_string())
                    } else {
                        Err(format!("{} {}", r.status, r.text().unwrap_or_default()))
                    }
                });
            *slot.lock().unwrap() = Some(parsed);
            ctx.request_repaint();
        });
    }

    fn rebuild(&mut self, data: &GraphData) {
        let set: BTreeSet<_> = data.nodes.iter().map(|n| n.partition.clone()).collect();
        self.partitions = set.into_iter().collect();
        if self
            .partition
            .as_ref()
            .is_some_and(|p| !self.partitions.contains(p))
        {
            self.partition = None;
        }
        self.graph = Some(Graph::new(data, self.partition.as_deref()));
        self.selected = None;
        self.status = format!(
            "{} nodes, {} synapses",
            data.nodes.len(),
            data.synapses.len()
        );
    }
}

fn role_color(role: &str) -> Color32 {
    match role {
        "user" => Color32::from_rgb(90, 160, 240),
        "assistant" => Color32::from_rgb(120, 200, 140),
        "system" => Color32::from_rgb(230, 170, 80),
        _ => Color32::from_rgb(180, 180, 190),
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let arrived = self.loaded.lock().unwrap().take();
        let mut data_for_rebuild = None;
        match arrived {
            Some(Ok(data)) => data_for_rebuild = Some(data),
            Some(Err(e)) => self.status = format!("Failed to load: {e}"),
            None => {}
        }
        if let Some(data) = data_for_rebuild {
            self.rebuild(&data);
            self.data = Some(data);
        }

        egui::Panel::top("controls").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Reservoir");
                ui.separator();
                let before = self.partition.clone();
                egui::ComboBox::from_label("Partition")
                    .selected_text(self.partition.as_deref().unwrap_or("all"))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.partition, None, "all");
                        for p in &self.partitions {
                            ui.selectable_value(&mut self.partition, Some(p.clone()), p);
                        }
                    });
                if before != self.partition {
                    if let Some(data) = self.data.take() {
                        self.rebuild(&data);
                        self.data = Some(data);
                    }
                }
                ui.checkbox(&mut self.show_traces, "Trace links");
                if ui.button("Reload").clicked() {
                    self.fetch(&ctx);
                }
                if ui.button("Reset view").clicked() {
                    self.zoom = 1.0;
                    self.pan = Vec2::ZERO;
                }
                ui.separator();
                ui.label(&self.status);
            });
        });

        if let (Some(graph), Some(i)) = (&self.graph, self.selected) {
            let n = &graph.nodes[i];
            egui::Panel::right("details")
                .default_size(320.0)
                .show(ui, |ui| {
                    ui.heading(&n.role);
                    ui.label(format!("{} / {}", n.partition, n.instance));
                    ui.label(format!("trace {}", n.trace_id));
                    ui.label(format!("id {}", n.id));
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(n.content.as_deref().unwrap_or(""));
                    });
                });
        }

        egui::CentralPanel::default().show(ui, |ui| {
            let Some(graph) = &mut self.graph else {
                return;
            };
            let (rect, resp) = ui.allocate_exact_size(ui.available_size(), Sense::click_and_drag());
            if resp.dragged() {
                self.pan += resp.drag_delta();
            }
            if resp.hovered() {
                let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                if scroll != 0.0 {
                    let f = (scroll * 0.005).exp();
                    let new = (self.zoom * f).clamp(0.1, 10.0);
                    if let Some(p) = resp.hover_pos() {
                        let c = rect.center().to_vec2() + self.pan;
                        let k = new / self.zoom;
                        self.pan += (p.to_vec2() - c) * (1.0 - k);
                    }
                    self.zoom = new;
                }
            }

            if graph.step() {
                ctx.request_repaint();
            }

            let center = rect.center() + self.pan;
            let zoom = self.zoom;
            let to_screen = |p: [f32; 2]| center + Vec2::new(p[0], p[1]) * zoom;
            let painter = ui.painter_at(rect);

            if self.show_traces {
                for &(a, b) in &graph.trace_links {
                    painter.line_segment(
                        [to_screen(graph.pos[a]), to_screen(graph.pos[b])],
                        Stroke::new(1.0, Color32::from_rgba_unmultiplied(160, 160, 180, 60)),
                    );
                }
            }
            for &(a, b, score) in &graph.synapse_links {
                let w = 1.0 + 2.5 * score.clamp(0.0, 1.0);
                painter.line_segment(
                    [to_screen(graph.pos[a]), to_screen(graph.pos[b])],
                    Stroke::new(w, Color32::from_rgb(200, 150, 240)),
                );
            }

            let hover = resp.hover_pos();
            let mut hovered = None;
            for (i, n) in graph.nodes.iter().enumerate() {
                let p = to_screen(graph.pos[i]);
                if !rect.contains(p) {
                    continue;
                }
                let r = 5.0 * zoom.sqrt();
                if hover.is_some_and(|h| (h - p).length() <= r + 2.0) {
                    hovered = Some(i);
                }
                let stroke = if self.selected == Some(i) {
                    Stroke::new(2.0, Color32::WHITE)
                } else {
                    Stroke::NONE
                };
                painter.circle(p, r, role_color(&n.role), stroke);
            }
            if let Some(i) = hovered {
                if resp.clicked() {
                    self.selected = Some(i);
                }
                let n = &graph.nodes[i];
                let preview: String = n.content.clone().unwrap_or_default().chars().take(160).collect();
                egui::Tooltip::always_open(
                    ctx.clone(),
                    ui.layer_id(),
                    egui::Id::unique("node-tip"),
                    egui::PopupAnchor::Pointer,
                )
                .show(|ui| {
                    ui.strong(format!("{} · {}", n.role, n.partition));
                    ui.label(preview);
                });
            } else if resp.clicked() {
                self.selected = None;
            }
        });
    }
}
