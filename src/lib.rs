use filtering::{evaluate_expr, parse_boolean_expr, ParsingError};
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    Graph,
};
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod app;
pub mod filtering;
pub mod vault_parser;

/// This struct stores the Markdown page information.
/// 
/// Pages are made when the [vault_parser] extracts the contents of a vault,
/// which then act as node weights in a [Graph](petgraph::Graph) instance.
/// [Nodes](Node) handle the graphical representations of Pages.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Page {
    /// Title of the page
    pub title: String,
    /// Tags of the page
    pub tags: Vec<String>,
    /// Whether the page is empty
    pub empty: bool,
    links: Vec<String>,
}

/// This struct stores the graphical representation of a [Page].
/// 
/// Nodes are used by the [GraphView] struct to keep track of the graph layout, filtering and grouping. 
pub struct Node {
    /// Node index corresponding to the [Page] in the [Graph](petgraph::Graph)
    pub node_index: NodeIndex,
    /// Position in pixels (relative to frame center)
    pub frame_pos: egui::Vec2, 
    /// Whether the node is visible (for filtering, etc.)
    pub visible: bool,
}

/// This struct handles the graphical representation of the node graph.
pub struct GraphView {
    graph: Graph<Page, ()>,
    nodes: HashMap<NodeIndex, Node>,
}

impl Node {
    pub fn new(node_index: NodeIndex, pos: egui::Vec2) -> Self {
        Self {
            node_index: node_index,
            frame_pos: pos,
            visible: true,
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

    /// Return a vector of all node positions in screenspace, with their corresponding node index
    pub fn node_positions(&self) -> Vec<(NodeIndex, egui::Vec2)> {
        self.nodes
            .iter()
            .map(|(index, node)| (*index, node.frame_pos))
            .collect()
    }

    /// Return a vector of edge start and end positions in screenspace
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

    /// Returns a copy of the associated page of a node
    pub fn node_page(&self, index: &NodeIndex) -> Option<Page> {
        if let Some(page) = self.graph.node_weight(*index) {
            Some(page.clone())
        } else {
            None
        }
    }

    /// Checks if the associated page of a node is empty
    pub fn node_is_empty(&self, index: NodeIndex) -> bool {
        if let Some(page) = self.graph.node_weight(index) {
            page.empty
        } else {
            true
        }
    }

    /// Returns the title of the associated page of a node
    pub fn node_title(&self, index: NodeIndex) -> String {
        if let Some(page) = self.graph.node_weight(index) {
            page.title.to_string()
        } else {
            "".to_string()
        }
    }

    /// Returns the tags of the associated page of a node
    pub fn node_tags(&self, index: NodeIndex) -> Vec<String> {
        if let Some(page) = self.graph.node_weight(index) {
            page.tags.clone()
        } else {
            Vec::default()
        }
    }

    /// Returns the visibility of a node
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

    /// Set node visibilities based on a filtering expression
    pub fn filter_nodes(&mut self, filter_query: &str) -> Option<ParsingError> {
        let expr_result = parse_boolean_expr(&filter_query);

        // Check if bool_expr is parsed successfully
        match expr_result {
            Ok(bool_expr) => {
                let mut empty_pages: Vec<(NodeIndex, Page)> = Vec::new(); // Evaluate empty pages last

                // Iterate over nodes and evaluate non-empty pages, store empty pages
                for (node_index, node) in &mut self.nodes {
                    // Extract page from node and evaluate expression
                    if let Some(page) = self.graph.node_weight(*node_index) {
                        if !page.empty {
                            if evaluate_expr(&bool_expr, &page) {
                                node.visible = true
                            } else {
                                node.visible = false
                            }
                        } else {
                            empty_pages.push((*node_index, page.clone()))
                        }
                    }
                }

                // Evaluate empty pages based on parent node
                for (node_index, _page) in empty_pages {
                    let parent_node_index = self
                        .graph
                        .neighbors_undirected(node_index)
                        .nth(0)
                        .expect("Empty node with no parent");
                    let parent_is_visible = self.nodes.get(&parent_node_index).unwrap().visible;
                    let mut node = self.nodes.get_mut(&node_index).unwrap();

                    if parent_is_visible {
                        node.visible = true
                    } else {
                        node.visible = false
                    }
                    /*
                    if evaluate_expr(&bool_expr, &page) {
                        node.visible = true
                    } else {
                        node.visible = false
                    }
                    */
                }

                None
            }
            Err(parsing_error) => Some(parsing_error),
        }
    }

    /// Advance the physics by 1 timestep
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
            if node.visible {
                let mut accel: egui::Vec2 = egui::Vec2::new(0., 0.);

                // Retrieve neighbor coordinates
                let neighbors: Vec<egui::Vec2> = self
                    .graph
                    .neighbors_undirected(*index)
                    .filter_map(|neigbor_node_index| {
                        let neighbor_node = self.nodes.get(&neigbor_node_index).unwrap();
                        if neighbor_node.visible {
                            Some(neighbor_node.frame_pos)
                        } else {
                            None
                        }
                    })
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
                    .filter_map(|(_, other_node)| {
                        if other_node.visible {
                            Some(other_node.frame_pos)
                        } else {
                            None
                        }
                    })
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
                        / gravity_switch_radius.powf(
                            gravity_force_exponent_primary - gravity_force_exponent_secondary,
                        )
                        * -node.frame_pos
                        / node
                            .frame_pos
                            .length()
                            .powf(gravity_force_exponent_secondary);
                }

                node_accel.push(accel)
            } else {
                node_accel.push(egui::Vec2::new(0., 0.))
            }
        }

        // Apply node accelerations
        for ((_, node), accel) in self.nodes.iter_mut().zip(node_accel) {
            node.frame_pos += accel * dt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_testing_graph() -> (NodeIndex, NodeIndex, NodeIndex, NodeIndex, Graph<Page, ()>) {
        // Create graph
        let mut graph = Graph::<Page, ()>::new();
        let page1 = graph.add_node(Page {
            title: "Page 1".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            empty: false,
            links: vec!["Page 2".to_string()],
        });
        let page2 = graph.add_node(Page {
            title: "Page 2".to_string(),
            tags: Vec::default(),
            empty: false,
            links: Vec::default(),
        });
        let page3 = graph.add_node(Page {
            title: "Page 3".to_string(),
            tags: Vec::default(),
            empty: false,
            links: vec!["Page 1".to_string(), "Page 2".to_string()],
        });
        let page4 = graph.add_node(Page {
            title: "Page 4".to_string(),
            tags: vec!["tag1".to_string()],
            empty: false,
            links: vec!["Page 3".to_string()],
        });

        // Create & filter graphview
        (page1, page2, page3, page4, graph)
    }

    #[test]
    fn graph_filtering_test_1() {
        let (page1, page2, page3, page4, graph) = create_testing_graph();
        let mut graphview = GraphView::new(graph);

        graphview.filter_nodes("tag:#tag1");

        assert!(
            graphview.node_is_visible(page1)
                && !graphview.node_is_visible(page2)
                && !graphview.node_is_visible(page3)
                && graphview.node_is_visible(page4)
        )
    }

    #[test]
    fn graph_filtering_test_2() {
        let (page1, page2, page3, page4, graph) = create_testing_graph();
        let mut graphview = GraphView::new(graph);

        graphview.filter_nodes("tag:#tag1 & tag:#tag2");

        assert!(
            graphview.node_is_visible(page1)
                && !graphview.node_is_visible(page2)
                && !graphview.node_is_visible(page3)
                && !graphview.node_is_visible(page4)
        )
    }
}
