use petgraph::{graph::NodeIndex, Graph};
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// This struct stores the Markdown page information
#[derive(Debug, Deserialize, Serialize)]
struct Page {
    title: String,
    tags: Vec<String>,
    empty: bool,
    links: Vec<String>,
}

// This struct stores the graphical representation of a node
pub struct Node {
    pub node_index: NodeIndex,
    pub frame_pos: egui::Vec2, // Relative to frame center
}

// This struct handles the graphical representation of the network
pub struct GraphView {
    graph: Graph<String, ()>,
    nodes: HashMap<NodeIndex, Node>,
}

impl Node {
    pub fn new(node_index: NodeIndex, pos: egui::Vec2) -> Self {
        Self {
            node_index: node_index,
            frame_pos: pos,
        }
    }
}

impl GraphView {
    pub fn new(graph: Graph<String, ()>) -> Self {
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

    // Returrn a vector of all node positions in screenspace
    pub fn node_positions(&self) -> Vec<(NodeIndex, egui::Vec2)> {
        self.nodes
            .iter()
            .map(|(index, node)| (*index, node.frame_pos))
            .collect()
    }

    // Return a vector of edge start and end positions
    pub fn edge_start_end_positions(&self) -> Vec<(egui::Vec2, egui::Vec2)> {
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

    // Return a vector of edge start and end positions
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

    // Modify a node position
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
                    //accel += repelling_constant/node_mass * (node.frame_pos - node_pos)/(node.frame_pos - node_pos).length().powf(1.5);
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
            /*
            if node.frame_pos.length() >= gravity_truncation_radius {
                accel += gravity_constant / node_mass * -node.frame_pos
                    / node.frame_pos.length().powf(gravity_force_exponent_primary);
            } else {
                accel += gravity_constant / node_mass * -node.frame_pos
                    / gravity_truncation_radius.powf(gravity_force_exponent_primary);
            }*/

            node_accel.push(accel)
        }

        //println!("{:?}", node_accel);

        // Apply node accelerations
        for ((_, node), accel) in self.nodes.iter_mut().zip(node_accel) {
            //node.frame_velocity += accel * dt;
            //node.frame_pos += node.frame_velocity * dt;
            node.frame_pos += accel * dt;
        }
    }
}

// This function reads the json file of the graph and converts it to a petgraph instance
pub fn json_graph(file_name: &str) -> Graph<String, ()> {
    // Read the JSON file into a string
    let mut file = File::open(file_name).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Parse the JSON string into a vector of Page structs
    let pages: Vec<Page> = serde_json::from_str(&contents).expect("Failed to parse JSON");

    // Create a directed graph
    let mut graph: Graph<String, ()> = Graph::new();
    let mut page_indices = std::collections::HashMap::new();

    // Add nodes (pages) and edges (links) to the graph
    for page in pages {
        let page_index = *page_indices.entry(page.title.clone()).or_insert_with(|| {
            let new_page_index = graph.add_node(page.title.clone());
            new_page_index
        });

        for link in page.links {
            let link_index = *page_indices.entry(link.clone()).or_insert_with(|| {
                let new_link_index = graph.add_node(link.clone());
                new_link_index
            });

            graph.add_edge(page_index, link_index, ());
        }
    }

    graph
}
