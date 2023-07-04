use obsidian_graph::{json_graph, GraphView};
use petgraph::{
    //dot::{Config, Dot},
    graph::NodeIndex,
    Graph,
};

use eframe::egui;

struct MyApp {
    // The graph currently being viewd
    graph: GraphView,
    // The coordinate of the center in screenspace
    frame_center: egui::Vec2,
    // If a node is currently being dragged
    dragging_node: Option<NodeIndex>, 
    // Current zoom level
    zoom: f32,
    // Zoom sensitivity
    zoom_step: f32,
    // Whether to draw arrows at node edges
    draw_arrows: bool,
    // Size of a node
    node_size: f32,
    // Width of links
    link_width: f32,
    // Whether physics is updated every frame
    enable_physics: bool,
    // Force strength to center of frame (gravity)
    center_force: f32,
    // Force strength repelling close nodes (electrostatic)
    repellant_force: f32,
    // Force strength between linked nodes (spring force)
    link_force: f32,
    // "Unstretched" link length
    link_length: f32,
    // Exponent of inverse-square law for gravity & electrostatic force
    force_exponent: f32,
    // Simulation timestep
    timestep: f32,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>, graph: Graph<String, ()>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self {
            graph: GraphView::new(graph),
            frame_center: egui::Vec2::new(640., 372.),
            dragging_node: None,
            zoom: 1.0,
            zoom_step: 0.15,
            draw_arrows: false,
            node_size: 8.5,
            link_width: 1.0,
            enable_physics: true,
            center_force: 500.0, //232.3,
            repellant_force: 350.0, //121.8,
            link_force: 0.50, //1.15,
            link_length: 75.0, //100.0,
            force_exponent: 2.5,
            timestep: 0.300, //0.145
        }
    }
}

// Update MyApp
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel
        egui::TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New File").clicked() {}
                    if ui.button("Open File...").clicked() {}
                    ui.menu_button("Open Recent", |ui| {
                        if ui.button("Project 1").clicked() {};
                        if ui.button("Project 2").clicked() {};
                        if ui.button("Project 3").clicked() {};
                    });
                    ui.separator();
                    if ui.button("Save").clicked() {}
                    if ui.button("Save As...").clicked() {}
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {}
                    if ui.button("Redo").clicked() {}
                    ui.separator();
                    if ui.button("Preferences").clicked() {}
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("User Manual").clicked() {}
                    if ui.button("Project Gitea Page").clicked() {}
                });
            });
        });

        // Side panel
        egui::SidePanel::right("settings_panel")
            .min_width(200.0)
            .show(ctx, |ui| {
                egui::CollapsingHeader::new("Physics settings")
                .default_open(true).show(ui, |ui| {

                        ui.checkbox(&mut self.enable_physics, "Enable physics");

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80., 20.],
                                egui::DragValue::new(&mut self.timestep)
                                    .speed(0.001)
                                    .clamp_range(0.0..=0.8),
                            );
                            ui.label("Timestep");
                        });

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80., 20.],
                                egui::DragValue::new(&mut self.force_exponent)
                                    .speed(0.01)
                                    .clamp_range(0.5..=2.5),
                            );
                            ui.label("Force exponent");
                        });

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80., 20.],
                                egui::DragValue::new(&mut self.center_force)
                                    .speed(0.1)
                                    .clamp_range(0.0..=500.0),
                            );
                            ui.label("Center force");
                        });

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80., 20.],
                                egui::DragValue::new(&mut self.repellant_force)
                                    .speed(0.1)
                                    .clamp_range(0.0..=500.0),
                            );
                            ui.label("Repellant force");
                        });

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80., 20.],
                                egui::DragValue::new(&mut self.link_force)
                                    .speed(0.01)
                                    .clamp_range(0.0..=2.0),
                            );
                            ui.label("Link force");
                        });

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80., 20.],
                                egui::DragValue::new(&mut self.link_length)
                                    .speed(1.0)
                                    .clamp_range(0.0..=200.0),
                            );
                            ui.label("Link length");
                        });
                    });

                egui::CollapsingHeader::new("Graph settings")
                .default_open(true).show(ui, |ui| {

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80.0, 20.0], 
                                egui::DragValue::new(&mut self.node_size)
                                .speed(0.1)
                                .clamp_range(0.0..=15.0),
                            );
                            ui.label("Node size");
                        });

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80.0, 20.0], 
                                egui::DragValue::new(&mut self.link_width)
                                .speed(0.1)
                                .clamp_range(0.0..=5.0),
                            );
                            ui.label("Link width");
                        });

                        ui.checkbox(&mut self.draw_arrows, "Draw arrows");

                        if ui.button("Reset view").clicked() {
                            self.zoom = 1.0;
                            self.frame_center = egui::Vec2::new(640., 372.)
                        }
                    });

                egui::CollapsingHeader::new("Display settings")
                .default_open(true).show(ui, |ui| {
                    ui.label("...");
                });

                egui::CollapsingHeader::new("Filter settings")
                    .default_open(true).show(ui, |ui| {
                        ui.label("...");
                    });

                
            });

        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ctx.request_repaint();

                //let (_, rect) = ui.allocate_space(ui.available_size());
                self.custom_painting(ui);

                //let response = ui.allocate_response(ui.available_size(), egui::Sense::click());
                //if response.clicked() { println!("badf") }
                //ui.painter().rect_stroke(response.rect, 0.0, (1.0, egui::Color32::WHITE));
            });
        });
    }
}

// MyApp custom painting
impl MyApp {
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        // Allocate interactive graphing area and initiate a painter
        let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
        let painter = ui.painter_at(response.rect);

        let mouse_pos = response.hover_pos().unwrap_or(egui::Pos2::new(0.0, 0.0));
        //self.frame_center = response.rect.center().to_vec2();

        if self.enable_physics {
            self.graph.physics_timestep(
                1.0,
                self.center_force,
                self.repellant_force,
                self.link_force,
                self.link_length,
                self.force_exponent,
                self.timestep,
            );
        }

        // Draw edges
        match self.draw_arrows {
            true => {
                // Draw arrows
                for (start_pos, end_pos) in self.graph.edge_start_end_positions() {
                    painter.arrow(
                        (self.zoom * start_pos).to_pos2() + self.frame_center,
                        self.zoom * (end_pos - start_pos), //egui::Vec2::new(self.zoom * end_pos[0], self.zoom * end_pos[1]),
                        egui::Stroke::new(self.link_width, egui::Color32::from_rgb(155, 155, 155)),
                    )
                }
            }
            false => {
                // Draw lines
                for (start_pos, end_pos) in self.graph.edge_start_end_positions() {
                    painter.line_segment(
                        [
                            (self.zoom * start_pos).to_pos2() + self.frame_center,
                            (self.zoom * end_pos).to_pos2() + self.frame_center,
                        ],
                        egui::Stroke::new(self.link_width, egui::Color32::from_rgb(155, 155, 155)),
                    )
                }
            }
        }

        // Draw nodes
        for (_, node_pos) in self.graph.node_positions() {
            painter.circle_filled(
                (self.zoom * node_pos).to_pos2() + self.frame_center,
                self.zoom * self.node_size,
                egui::Color32::from_rgb(255, 105, 105),
            );
        }

        // Zoom graph area
        let scroll = ui.input(|i| i.scroll_delta);
        if scroll.y != 0. {
            let old_zoom = self.zoom;
            self.zoom *= 1.0 + self.zoom_step * scroll.y / 50.;

            self.frame_center.x -=
                (mouse_pos[0] - self.frame_center.x) * (self.zoom / old_zoom - 1.0);
            self.frame_center.y -=
                (mouse_pos[1] - self.frame_center.y) * (self.zoom / old_zoom - 1.0);
        }

        // Drag graph area or nodes
        if response.dragged() {
            // Check if a node is already being dragged
            match self.dragging_node {
                Some(dragging_node_index) => self.graph.set_node_position(
                    dragging_node_index,
                    (mouse_pos.to_vec2() - self.frame_center) / self.zoom,
                ),
                None => {
                    for (index, node_pos) in self.graph.node_positions() {
                        if ((self.zoom * node_pos) + self.frame_center - mouse_pos.to_vec2())
                            .length()
                            <= self.zoom * self.node_size
                        {
                            self.dragging_node = Some(index);

                            self.graph.set_node_position(
                                index,
                                (mouse_pos.to_vec2() - self.frame_center) / self.zoom,
                            )
                        }
                    }

                    // Check if still no node is being dragged, then drag screen
                    if self.dragging_node == None {
                        self.frame_center += response.drag_delta()
                    }
                }
            }
        } else {
            self.dragging_node = None
        }

        /*
        if let Some(position) = response.interact_pointer_pos() {
            println!("{:?}", position);
        }
        */
    }
}

fn main() -> eframe::Result<()> {
    let graph = json_graph("graph.json");
    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Obsidian graph analiser",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc, graph))),
    )
}
