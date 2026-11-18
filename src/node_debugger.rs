use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Serialize, Deserialize)]
pub struct NodeSnapshot {
    pub name: String,
    pub depth: usize,
    pub size: f64,
    pub top_left_x: f64,
    pub top_left_y: f64,
    pub particles: Vec<usize>,
    pub children: Vec<NodeSnapshot>,
}

pub struct NodeDebugger {
    root: Option<NodeSnapshot>,
}

impl Default for NodeDebugger {
    fn default() -> Self {
        Self { root: None }
    }
}

impl eframe::App for NodeDebugger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Read the latest tree snapshot
        if let Ok(data) = fs::read_to_string("node_tree.json") {
            if let Ok(root) = serde_json::from_str::<NodeSnapshot>(&data) {
                self.root = Some(root);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Quadtree Debugger");
            ui.separator();

            if let Some(root) = &self.root {
                ui.label(format!("Total particles: {}", count_particles(root)));

                ui.label(format!("Total nodes: {}", count_nodes(root)));

                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    draw_node(ui, root);
                });
            } else {
                ui.label("Waiting for simulation...");
            }
        });

        // Refresh the window regularly
        ctx.request_repaint_after(std::time::Duration::from_millis(200));
    }
}

fn draw_node(ui: &mut egui::Ui, node: &NodeSnapshot) {
    let label = format!(
        "{} | {} particles | size {:.2e}",
        node.name,
        node.particles.len(),
        node.size
    );

    if node.children.is_empty() {
        ui.label(label);
        return;
    }

    egui::CollapsingHeader::new(label)
        .default_open(node.depth < 2)
        .show(ui, |ui| {
            ui.label(format!("Depth: {}", node.depth));

            ui.label(format!(
                "Top-left: ({:.2e}, {:.2e})",
                node.top_left_x, node.top_left_y
            ));

            ui.label(format!("Particle indices: {:?}", node.particles));

            for child in &node.children {
                draw_node(ui, child);
            }
        });
}

fn count_nodes(node: &NodeSnapshot) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}

fn count_particles(node: &NodeSnapshot) -> usize {
    node.particles.len() + node.children.iter().map(count_particles).sum::<usize>()
}

pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Node Debugger")
            .with_inner_size([600.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Node Debugger",
        options,
        Box::new(|_cc| Ok(Box::new(NodeDebugger::default()))),
    )
    .unwrap();
}
use crate::quadtree::Node;

pub fn snapshot_node(node: &Node, name: &str) -> NodeSnapshot {
    let names = ["TL", "TR", "BL", "BR"];

    let children = match &node.children {
        Some(children) => children
            .iter()
            .enumerate()
            .map(|(i, child)| snapshot_node(child, names[i]))
            .collect(),
        None => Vec::new(),
    };

    NodeSnapshot {
        name: name.to_string(),
        depth: node.depth,
        size: node.size,
        top_left_x: node.top_left.x,
        top_left_y: node.top_left.y,
        particles: node.particles.clone(),
        children,
    }
}
