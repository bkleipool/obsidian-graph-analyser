use crate::{filtering::ParsingError, vault_parser::vault_to_graph, GraphView, Page};
use eframe::egui;
use petgraph::{
    // dot::{Config, Dot},
    graph::NodeIndex,
    Graph,
};

pub struct MyApp {
    /// The graph currently being viewd
    graphview: GraphView,
    /// The coordinate of the center in screenspace
    frame_center: egui::Vec2,
    /// The size of the drawing area in screenspace (automatically updating)
    frame_size: egui::Vec2,
    /// If a node is currently being dragged
    dragging_node: Option<NodeIndex>,
    /// If a node is currently being hovered over
    hovering_node: Option<NodeIndex>,
    /// The number of frames while which a node is being hovered over
    node_hover_time: f32,
    /// Current zoom level
    zoom: f32,
    /// Zoom sensitivity
    zoom_step: f32,
    /// Whether to draw arrows at node edges
    draw_arrows: bool,
    /// Whether to draw node labels
    draw_labels: bool,
    /// Radius of a node (px)
    node_size: f32,
    /// Width of links (px)
    link_width: f32,
    /// Size of link arrows (if enabled)
    arrow_size: f32,
    /// Size of node label (pt)
    text_size: f32,
    /// Size of node label at which to start fading the text (pt)
    text_fade_threshold: f32,
    /// Whether physics is updated every frame
    enable_physics: bool,
    /// Force strength to center of frame (gravity)
    gravity_force: f32,
    /// Force strength repelling close nodes (electrostatic)
    repellant_force: f32,
    /// Force strength between linked nodes (spring force)
    link_force: f32,
    /// "Unstretched" link length
    link_length: f32,
    /// Exponent of inverse-square law for electrostatic force
    repelling_force_exponent: f32,
    /// Exponent of inverse-square law for gravity force (close to center)
    gravity_force_exponent_primary: f32,
    /// Exponent of inverse-square law for gravity force (far from center)
    gravity_force_exponent_secondary: f32,
    ///  The radius from center at which the gravity force exponent switches from primary to secondary
    gravity_switch_radius: f32,
    /// The radius from center at which the gravity force is truncated from the primary inverse-square law value
    gravity_truncation_radius: f32,
    /// Simulation timestep
    timestep: f32,
    /// Query used when filtering nodes in the graph
    filter_query: String,
    /// Error encountered when parsing filtering expression (if any)
    filtering_error: Option<ParsingError>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, graph: Graph<Page, ()>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self {
            graphview: GraphView::new(graph),
            frame_center: egui::Vec2::new(640., 372.),
            frame_size: egui::Vec2::new(0., 0.),
            dragging_node: None,
            hovering_node: None,
            node_hover_time: 0.,
            zoom: 1.0,
            zoom_step: 0.15,
            draw_arrows: false,
            draw_labels: true,
            node_size: 8.5,
            link_width: 1.0,
            arrow_size: 8.0,
            text_size: 8.0,
            text_fade_threshold: 4.5,

            enable_physics: true,
            gravity_force: 400.0,
            repellant_force: 350.0,
            link_force: 0.50,
            link_length: 100.0,
            repelling_force_exponent: 2.5,
            gravity_force_exponent_primary: 1.9,
            gravity_force_exponent_secondary: 0.75,
            gravity_switch_radius: 1000.0,
            gravity_truncation_radius: 350.0,
            timestep: 0.400,

            filter_query: String::default(),
            filtering_error: None,
        }
    }
}

// Update MyApp
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Look for drag-and-dropped files
        self.ui_file_drag_and_drop(ctx);

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
                    if ui.button("Preferences").clicked() {}
                });
                //ui.menu_button("Edit", |ui| {
                //    if ui.button("Undo").clicked() {}
                //    if ui.button("Redo").clicked() {}
                //});
                ui.menu_button("Help", |ui| {
                    ui.hyperlink_to(
                        "Project Github Page",
                        "https://github.com/bkleipool/obsidian-graph-analyser",
                    );
                    if ui.button("Documentation").clicked() {}
                });
            });
        });

        // Side panel
        egui::SidePanel::right("settings_panel")
            .min_width(200.0)
            .show(ctx, |ui| {
                egui::CollapsingHeader::new("Physics settings")
                    .default_open(true)
                    .show(ui, |ui| {
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
                                egui::DragValue::new(&mut self.repelling_force_exponent)
                                    .speed(0.01)
                                    .clamp_range(0.5..=2.5),
                            );
                            ui.label("Repelling force exponent");
                        });

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80., 20.],
                                egui::DragValue::new(&mut self.gravity_force_exponent_primary)
                                    .speed(0.01)
                                    .clamp_range(0.5..=2.5),
                            );
                            ui.label("1st Gravity force exponent"); // (close to center)
                        });

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80., 20.],
                                egui::DragValue::new(&mut self.gravity_force_exponent_secondary)
                                    .speed(0.01)
                                    .clamp_range(0.5..=2.5),
                            );
                            ui.label("2nd Gravity force exponent"); //(far from center)
                        });

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80., 20.],
                                egui::DragValue::new(&mut self.gravity_force)
                                    .speed(0.1)
                                    .clamp_range(0.0..=500.0),
                            );
                            ui.label("Gravity force");
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
                                egui::DragValue::new(&mut self.gravity_truncation_radius)
                                    .speed(1.0)
                                    .clamp_range(0.0..=500.0),
                            );
                            ui.label("Gravity truncation radius");
                        });

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80., 20.],
                                egui::DragValue::new(&mut self.gravity_switch_radius)
                                    .speed(5.0)
                                    .clamp_range(0.0..=2000.0),
                            );
                            ui.label("Gravity switch radius");
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
                    .default_open(true)
                    .show(ui, |ui| {
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

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80.0, 20.0],
                                egui::DragValue::new(&mut self.arrow_size)
                                    .speed(0.1)
                                    .clamp_range(0.0..=15.0),
                            );
                            ui.label("Arrow size");
                        });

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80.0, 20.0],
                                egui::DragValue::new(&mut self.text_size)
                                    .speed(0.1)
                                    .clamp_range(0.0..=16.0),
                            );
                            ui.label("Text size");
                        });

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [80.0, 20.0],
                                egui::DragValue::new(&mut self.text_fade_threshold)
                                    .speed(0.1)
                                    .clamp_range(0.0..=self.text_size),
                            );
                            ui.label("Text fade threshold");
                        });

                        ui.checkbox(&mut self.draw_arrows, "Draw arrows");

                        ui.checkbox(&mut self.draw_labels, "Draw labels");

                        if ui.button("Reset view").clicked() {
                            self.zoom = 1.0;
                            self.frame_center = egui::Vec2::new(640., 372.)
                        }

                        // ui.add(egui::DragValue::new(&mut self.node_hover_time));
                    });

                egui::CollapsingHeader::new("Filtering settings")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let response = ui.add_sized(
                                [150.0, 20.0],
                                egui::TextEdit::singleline(&mut self.filter_query),
                            );
                            ui.label("Filtering");

                            if response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                            {
                                self.filtering_error =
                                    self.graphview.filter_nodes(&self.filter_query);
                            }
                        });

                        if let Some(parsing_error) = &self.filtering_error {
                            let text = match parsing_error {
                                ParsingError::InvalidExpression(expr) => {
                                    "Invalid expression: ".to_owned() + expr
                                }
                                ParsingError::MissingOperand(expr) => {
                                    "Missing operand: ".to_owned() + expr
                                }
                                ParsingError::MissingOperator(expr) => {
                                    "Missing operator: ".to_owned() + expr
                                }
                                ParsingError::UnmatchedParentheses => {
                                    "Unmatched parentheses".to_string()
                                }
                            };
                            ui.label(text);
                        } else {
                            ui.label("");
                        }

                        if ui.button("Show all nodes").clicked() {
                            for (_, node) in self.graphview.nodes.iter_mut() {
                                node.visible = true
                            }
                        }
                    });

                egui::CollapsingHeader::new("Filter settings")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.label("...");
                    });
            });

        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ctx.request_repaint();

                self.custom_painting(ui);
            });
        });
    }
}

// MyApp custom painting
impl MyApp {
    // Graph view
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        // Allocate interactive graphing area and initiate a painter
        let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
        let painter = ui.painter_at(response.rect);

        let mouse_pos = response.hover_pos().unwrap_or(egui::Pos2::new(0.0, 0.0));
        self.frame_size = response.rect.size();

        // Perform physics timestep
        if self.enable_physics {
            self.graphview.physics_timestep(
                1.0,
                self.gravity_force,
                self.repellant_force,
                self.link_force,
                self.link_length,
                self.repelling_force_exponent,
                self.gravity_force_exponent_primary,
                self.gravity_force_exponent_secondary,
                self.gravity_switch_radius,
                self.gravity_truncation_radius,
                self.timestep,
            );
        }

        // Draw edges
        match self.draw_arrows {
            true => {
                // Draw arrows
                for (_, start_pos, end_pos) in self.graphview.edge_start_end_positions() {
                    let dir = (end_pos - start_pos).normalized();

                    let origin = (self.zoom * start_pos).to_pos2() + self.frame_center;
                    let tip = origin + self.zoom * (end_pos - start_pos - self.node_size * dir);

                    let stroke =
                        egui::Stroke::new(self.link_width, egui::Color32::from_rgb(155, 155, 155));
                    let angle = egui::emath::Rot2::from_angle(std::f32::consts::TAU / 10.0);

                    let tip_length = match self.zoom * (end_pos - start_pos).length() {
                        x if x / 8.0 < self.arrow_size => {
                            self.zoom * (end_pos - start_pos).length() / 8.0
                        }
                        _ => self.arrow_size,
                    };

                    painter.line_segment([origin, tip], stroke);
                    painter.line_segment([tip, tip - tip_length * (angle * dir)], stroke);
                    painter.line_segment([tip, tip - tip_length * (angle.inverse() * dir)], stroke);
                }
            }
            false => {
                // Draw lines
                for (edge_index, start_pos, end_pos) in self.graphview.edge_start_end_positions() {
                    let (edge_start_node, edge_end_node) =
                        self.graphview.graph.edge_endpoints(edge_index).unwrap();

                    if self.graphview.node_is_visible(edge_start_node)
                        && self.graphview.node_is_visible(edge_end_node)
                    {
                        // Check if edge is connected to hovering node
                        if Some(edge_start_node) == self.hovering_node
                            || Some(edge_end_node) == self.hovering_node
                        {
                            painter.line_segment(
                                [
                                    (self.zoom * start_pos).to_pos2() + self.frame_center,
                                    (self.zoom * end_pos).to_pos2() + self.frame_center,
                                ],
                                egui::Stroke::new(
                                    self.link_width,
                                    egui::Color32::from_rgb(255, 105, 105),
                                ),
                            )
                        } else {
                            painter.line_segment(
                                [
                                    (self.zoom * start_pos).to_pos2() + self.frame_center,
                                    (self.zoom * end_pos).to_pos2() + self.frame_center,
                                ],
                                egui::Stroke::new(
                                    self.link_width,
                                    egui::Color32::from_rgb(155, 155, 155),
                                ),
                            )
                        }
                    }
                }
            }
        }

        // Draw nodes
        for (node_index, node_pos) in self.graphview.node_positions() {
            if self.graphview.node_is_visible(node_index) {
                // Check if node is being hovered
                if Some(node_index) != self.hovering_node {
                    // Check if node is not empty
                    if !self.graphview.node_is_empty(node_index) {
                        painter.circle_filled(
                            (self.zoom * node_pos).to_pos2() + self.frame_center,
                            self.zoom * self.node_size,
                            egui::Color32::from_rgb(200, 200, 200),
                        )
                    } else {
                        painter.circle_filled(
                            (self.zoom * node_pos).to_pos2() + self.frame_center,
                            self.zoom * self.node_size,
                            egui::Color32::from_rgb(50, 50, 50),
                        )
                    }
                } else {
                    painter.circle_filled(
                        (self.zoom * node_pos).to_pos2() + self.frame_center,
                        self.zoom * self.node_size,
                        egui::Color32::from_rgb(255, 105, 105),
                    )
                }
            }
        }

        // Draw node labels (if they are visible on the screen)
        if self.draw_labels {
            for (node_index, node_pos) in self.graphview.node_positions() {
                let text_pos = (self.zoom * (node_pos + egui::Vec2::new(0., self.node_size + 2.0)))
                    + self.frame_center;

                if (0.0..=self.frame_size.x).contains(&text_pos.x)
                    && (0.0..=self.frame_size.y).contains(&text_pos.y)
                    && self.graphview.node_is_visible(node_index)
                {
                    painter.text(
                        text_pos.to_pos2(),
                        egui::Align2::CENTER_TOP,
                        self.graphview.node_title(node_index),
                        egui::FontId::proportional(self.text_size * self.zoom),
                        egui::Color32::from_rgba_unmultiplied(
                            255,
                            255,
                            255,
                            match self.zoom {
                                x if (0.0..=self.text_fade_threshold / self.text_size)
                                    .contains(&x) =>
                                {
                                    0
                                }
                                x if (self.text_fade_threshold / self.text_size..=1.0)
                                    .contains(&x) =>
                                {
                                    (255. / (1.0 - self.text_fade_threshold / self.text_size)
                                        * (self.zoom - self.text_fade_threshold / self.text_size))
                                        as u8
                                }
                                _ => 255,
                            },
                        ),
                    );
                }
            }
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
                Some(dragging_node_index) => self.graphview.set_node_position(
                    dragging_node_index,
                    (mouse_pos.to_vec2() - self.frame_center) / self.zoom,
                ),
                None => {
                    for (index, node_pos) in self.graphview.node_positions() {
                        if ((self.zoom * node_pos) + self.frame_center - mouse_pos.to_vec2())
                            .length()
                            <= self.zoom * self.node_size
                        {
                            self.dragging_node = Some(index);

                            self.graphview.set_node_position(
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

        // Hover over node
        let old_node_hover_time = self.node_hover_time;

        if response.hovered() && self.dragging_node == None {
            for (index, node_pos) in self.graphview.node_positions() {
                if ((self.zoom * node_pos) + self.frame_center - mouse_pos.to_vec2()).length()
                    <= self.zoom * self.node_size
                {
                    self.node_hover_time += 1.;
                    self.hovering_node = Some(index);

                    // ...
                    // ...
                }
            }

            if old_node_hover_time == self.node_hover_time {
                self.node_hover_time = 0.;
                self.hovering_node = None;
            }
        } else {
            self.node_hover_time = 0.;
            self.hovering_node = None;
        }

        /*
        if let Some(position) = response.interact_pointer_pos() {
            println!("{:?}", position);
        }
        */
    }

    // Drag & drop vaults
    fn ui_file_drag_and_drop(&mut self, ctx: &egui::Context) {
        use egui::*;
        use std::fmt::Write as _;

        // Preview hovering file
        if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
            let text = ctx.input(|i| {
                let mut text = "Dropping files:\n".to_owned();
                let file_path = &i.raw.hovered_files[0].path;
                write!(text, "\n{}", file_path.as_ref().unwrap().display()).ok();

                text
            });

            let painter =
                ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

            let screen_rect = ctx.screen_rect();
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                TextStyle::Heading.resolve(&ctx.style()),
                Color32::WHITE,
            );
        }

        // Show dropped file
        ctx.input(|i| {
            let dropped_file = if !i.raw.dropped_files.is_empty() {
                i.raw.dropped_files[0].clone()
            } else {
                egui::DroppedFile::default()
            };

            if dropped_file != egui::DroppedFile::default() {
                if let Some(path) = &dropped_file.path {
                    if path.is_dir() {
                        self.graphview = GraphView::new(vault_to_graph(path))
                    }
                };
            };
        });
    }
}
