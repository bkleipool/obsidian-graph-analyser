use eframe::egui;
use obsidian_graph::{app::MyApp, json_graph};
// use petgraph::dot::{Config, Dot};


fn main() -> eframe::Result<()> {
    let graph = json_graph("graph2.json");
    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    //println!("{:?}",graph.node_count());

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
