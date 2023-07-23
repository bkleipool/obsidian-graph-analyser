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

    /// Returns a copy of the associated page of a node
    pub fn node_page(&self, index: &NodeIndex) -> Option<Page> {
        if let Some(page) = self.graph.node_weight(*index) {
            Some(page.clone())
        } else {
            None
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




// (A & tag:#B) | (C & -tag:#D) -> expression (describes boolean logic operations)
// tag:#B -> filter (describes specific field which is filtered)
// B -> query (regex which returns true / false)


// Define the boolean expression tree data structure
#[derive(Debug)]
pub enum BooleanExpr {
    Not(Box<BooleanExpr>),
    And(Box<BooleanExpr>, Box<BooleanExpr>),
    Or(Box<BooleanExpr>, Box<BooleanExpr>),
    Filter(String),
}

#[derive(Debug)]
pub enum ParsingError {
    UnmatchedParentheses,
    MissingOperand(String),
    MissingOperator(String),
    InvalidExpression(String),
}

// Helper function to check if a character is a valid query character
fn is_valid_char(c: char) -> bool {
    c != '(' && c != ')' && c != '&' && c != '|' && c != '-' && c != ' ' 
}

/// Turns a string into a boolean syntax tree (recursively)
/// & = AND operator
/// | = OR operator
/// - = NOT operator
pub fn parse_boolean_expr(expr: &str) -> Result<BooleanExpr, ParsingError> {
    let expr = expr.trim();

    // Check for the base case (single variable)
    if expr.chars().all(is_valid_char) {
        return Ok(BooleanExpr::Filter(expr.to_string()));
    }

    // Check for NOT operator
    if expr.starts_with('-') {
        let inner_expr = parse_boolean_expr(&expr[1..])?;
        return Ok(BooleanExpr::Not(Box::new(inner_expr)));
    }

    // Find the index where the AND or OR operator is located
    let mut idx = expr.len() - 1;
    let mut level = 0;

    while idx > 0 {
        let c = expr.chars().nth(idx).unwrap();
        match c {
            '(' => level -= 1,
            ')' => level += 1,
            '&' if level == 0 => break,
            '|' if level == 0 => break,
            _ => {}
        }
        idx -= 1;
    }

    if idx == 0 {
        // The expression is surrounded by parentheses, so we ignore them and parse the inner part
        if expr.starts_with('(') && expr.ends_with(')') {
            return parse_boolean_expr(&expr[1..expr.len() - 1]);
        }
        // Invalid expression
        if level != 0 {
            return Err(ParsingError::UnmatchedParentheses);
        } else if expr.chars().all(is_valid_char) {
            return Err(ParsingError::MissingOperator(expr.to_string()));
        } else {
            return Err(ParsingError::InvalidExpression(expr.to_string()));
        }
    }

    let left_expr = parse_boolean_expr(&expr[..idx])?;
    let right_expr = parse_boolean_expr(&expr[idx + 1..])?;

    match expr.chars().nth(idx).unwrap() {
        '&' => Ok(BooleanExpr::And(Box::new(left_expr), Box::new(right_expr))),
        '|' => Ok(BooleanExpr::Or(Box::new(left_expr), Box::new(right_expr))),
        _ => Err(ParsingError::InvalidExpression("Bruh2".to_string())),
    }
}

// Evaluates a (nested) boolean expression for some input [Page]
pub fn evaluate_expr(expr: &BooleanExpr, input: &Page) -> bool {
    match expr {
        BooleanExpr::Not(inner_expr) => {
            !evaluate_expr(inner_expr.as_ref(), input)
        },
        BooleanExpr::And(inner_expr_left, inner_expr_right) => {
            evaluate_expr(inner_expr_left.as_ref(), input) && evaluate_expr(inner_expr_right.as_ref(), input)
        },
        BooleanExpr::Or(inner_expr_left, inner_expr_right) => {
            evaluate_expr(inner_expr_left.as_ref(), input) || evaluate_expr(inner_expr_right.as_ref(), input)
        },
        BooleanExpr::Filter(filter_string) => {
            evaluate_filter(filter_string, input)
        },
    }
}

// Evaluates a filter for some input [Page]
fn evaluate_filter(filter: &str, input: &Page) -> bool {
    // Page filter
    if filter.starts_with("title:") {
        let query = filter.strip_prefix("title:").unwrap();
        input.title.contains(query)

    // Tag filter
    } else if filter.starts_with("tag:#") {
        let query = filter.strip_prefix("tag:#").unwrap();

        input.tags.iter().any(|tag| tag.contains(query))

    // Page filter
    } else {
        let query = filter;
        input.title.contains(query)
    }

}