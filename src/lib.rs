use petgraph::{graph::{NodeIndex, EdgeIndex}, Graph};
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod app;
pub mod vault_parser;

/// This struct stores the Markdown page information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Page {
    title: String,
    tags: Vec<String>,
    empty: bool,
    links: Vec<String>,
}

/// This struct stores the graphical representation of a [Page]
pub struct Node {
    pub node_index: NodeIndex,
    pub frame_pos: egui::Vec2, // Relative to frame center
    pub visible: bool,
}

/// This struct handles the graphical representation of the network
pub struct GraphView {
    graph: Graph<Page, ()>,
    nodes: HashMap<NodeIndex, Node>,
}

impl Node {
    pub fn new(node_index: NodeIndex, pos: egui::Vec2) -> Self {
        Self {
            node_index: node_index,
            frame_pos: pos,
            visible: true
        }
    }
}

impl GraphView {
    pub fn new(graph: Graph<Page, ()>) -> Self {
        // Save nodes to hashmap
        let nodes: HashMap<NodeIndex, Node> = graph
            .node_indices()
            .into_iter()
            .map(|i| {
                let normal = Normal::new(0.0, 100.0).unwrap();
                (
                    i,
                    Node::new(
                        i,
                        egui::Vec2::new(
                            normal.sample(&mut rand::thread_rng()),
                            normal.sample(&mut rand::thread_rng()),
                        ),
                    ),
                )
            })
            .collect();

        Self {
            graph: graph,
            nodes: nodes,
        }
    }

    /// Return a vector of all node positions in screenspace
    pub fn node_positions(&self) -> Vec<(NodeIndex, egui::Vec2)> {
        self.nodes
            .iter()
            .map(|(index, node)| (*index, node.frame_pos))
            .collect()
    }

    /// Return a vector of edge start and end positions
    pub fn edge_start_end_positions(&self) -> Vec<(EdgeIndex, egui::Vec2, egui::Vec2)> {
        self.graph
            .edge_indices()
            .into_iter()
            .map(|edge| {
                let (start_index, end_index) = self.graph.edge_endpoints(edge).unwrap();

                (
                    edge,
                    self.nodes.get(&start_index).unwrap().frame_pos,
                    self.nodes.get(&end_index).unwrap().frame_pos,
                )
            })
            .collect()
    }

    /// Return a vector of edge start and end positions
    pub fn edge_origin_magnitudes(&self) -> Vec<(egui::Vec2, egui::Vec2)> {
        self.graph
            .edge_indices()
            .into_iter()
            .map(|edge| {
                let (start_index, end_index) = self.graph.edge_endpoints(edge).unwrap();

                (
                    self.nodes.get(&start_index).unwrap().frame_pos,
                    self.nodes.get(&end_index).unwrap().frame_pos,
                )
            })
            .collect()
    }

    /// Checks if a node in the graph is empty
    pub fn node_is_empty(&self, index: NodeIndex) -> bool {
        if let Some(page) = self.graph.node_weight(index) {
            page.empty
        } else {
            true
        }
    }

    /// Returns the name of a node
    pub fn node_title(&self, index: NodeIndex) -> String {
        if let Some(page) = self.graph.node_weight(index) {
            page.title.to_string()
        } else {
            "".to_string()
        }
    }

    /// Returns the name of a node
    pub fn node_is_visible(&self, index: NodeIndex) -> bool {
        let node = self.nodes.get(&index).expect("Node not found");

        node.visible
    }

    /// Modify a node position
    pub fn set_node_position(&mut self, index: NodeIndex, pos: egui::Vec2) {
        if let Some(node) = self.nodes.get_mut(&index) {
            node.frame_pos = pos;
        }
    }

    pub fn physics_timestep(
        &mut self,
        node_mass: f32,
        gravity_constant: f32,
        repelling_constant: f32,
        spring_constant: f32,
        spring_length: f32,
        repelling_force_exponent: f32,
        gravity_force_exponent_primary: f32,
        gravity_force_exponent_secondary: f32,
        gravity_switch_radius: f32,
        gravity_truncation_radius: f32,
        dt: f32,
    ) {
        let mut node_accel: Vec<egui::Vec2> = Vec::new();

        // Calculate node accelerations
        for (index, node) in self.nodes.iter() {
            let mut accel: egui::Vec2 = egui::Vec2::new(0., 0.);

            // Retrieve neighbor coordinates
            let neighbors: Vec<egui::Vec2> = self
                .graph
                .neighbors_undirected(*index)
                .map(|node_index| self.nodes.get(&node_index).unwrap().frame_pos)
                .collect();

            // Get acceleration due to springs
            for neighbor_pos in neighbors.iter() {
                let neighbor_accel = spring_constant / node_mass
                    * (*neighbor_pos - node.frame_pos)
                    * (1.0 - spring_length / ((*neighbor_pos - node.frame_pos).length()) + 1.0)
                        .log10();

                if !neighbor_accel.any_nan() {
                    accel += neighbor_accel
                }
            }

            // Retrieve coordinates of all nodes
            let node_coords: Vec<egui::Vec2> = self
                .nodes
                .iter()
                .map(|(_, other_node)| other_node.frame_pos)
                .collect();

            // Get repellant forces
            for node_pos in node_coords {
                if (node.frame_pos - node_pos).length() >= 0.1 {
                    accel += repelling_constant / node_mass * (node.frame_pos - node_pos)
                        / ((node.frame_pos - node_pos)
                            .length()
                            .powf(repelling_force_exponent));
                }
            }

            // Add center acceleration
            if node.frame_pos.length() <= gravity_truncation_radius {
                accel += gravity_constant / node_mass * -node.frame_pos
                    / gravity_truncation_radius.powf(gravity_force_exponent_primary);
            } else if node.frame_pos.length() <= gravity_switch_radius {
                accel += gravity_constant / node_mass * -node.frame_pos
                    / node.frame_pos.length().powf(gravity_force_exponent_primary);
            } else {
                accel += gravity_constant / node_mass * 1.0
                    / gravity_switch_radius
                        .powf(gravity_force_exponent_primary - gravity_force_exponent_secondary)
                    * -node.frame_pos
                    / node
                        .frame_pos
                        .length()
                        .powf(gravity_force_exponent_secondary);
            }

            node_accel.push(accel)
        }

        // Apply node accelerations
        for ((_, node), accel) in self.nodes.iter_mut().zip(node_accel) {
            node.frame_pos += accel * dt;
        }
    }
}



use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "page_search/search_criteria.pest"]
struct FilterQueryParser;

impl GraphView {
    fn filter_pages(&mut self, search_query: &str) {
        // Parse filter query
        let pairs = FilterQueryParser::parse(Rule::query, search_query)
            .expect("Failed to parse search query")
            .next()
            .unwrap();


        // Set all nodes invisible
        for (_, node) in self.nodes.iter_mut() {
            node.visible = false
        }
    
        // Match filter queries
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::title_include => {
                    let term = pair.as_str().to_lowercase();
                    println!("include: {term}");

                    for (node_index, node) in self.nodes.iter_mut() {
                        let page = self.graph.node_weight(*node_index).unwrap();

                        if page.title.to_lowercase().contains(&term)
                            || page.tags.iter().any(|tag| tag.to_lowercase().contains(&term))
                        {
                            node.visible = true
                        }
                    }
                },

                Rule::title_exclude => {
                    let term = pair.as_str().to_lowercase()[1..].to_string();
                    println!("exclude: {term}");

                    for (node_index, node) in self.nodes.iter_mut() {
                        let page = self.graph.node_weight(*node_index).unwrap();

                        if !page.title.to_lowercase().contains(&term)
                            || page.tags.iter().any(|tag| tag.to_lowercase().contains(&term))
                        {
                            node.visible = true
                        }
                    }
                }

                Rule::title_include => {
                    let term = pair.as_str().to_lowercase();
                    println!("include: {term}");

                    for (node_index, node) in self.nodes.iter_mut() {
                        let page = self.graph.node_weight(*node_index).unwrap();

                        if page.title.to_lowercase().contains(&term)
                            || page.tags.iter().any(|tag| tag.to_lowercase().contains(&term))
                        {
                            node.visible = true
                        }
                    }
                },

                Rule::title_exclude => {
                    let term = pair.as_str().to_lowercase()[1..].to_string();
                    println!("exclude: {term}");

                    for (node_index, node) in self.nodes.iter_mut() {
                        let page = self.graph.node_weight(*node_index).unwrap();

                        if !page.title.to_lowercase().contains(&term)
                            || page.tags.iter().any(|tag| tag.to_lowercase().contains(&term))
                        {
                            node.visible = true
                        }
                    }
                }
                _ => {},
            }
        }
    
    }
}
