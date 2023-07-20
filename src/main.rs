use std::path::Path;
use eframe::egui;
use obsidian_graph::{app::MyApp, vault_parser::vault_to_graph};
// use petgraph::dot::{Config, Dot};


fn main() -> eframe::Result<()> {
    let graph = vault_to_graph(&Path::new("test_vault"));
    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    //println!("{:?}",graph.node_count());
    //println!("{:?}", search_markdown_files(&Path::new("test_vault")));

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Obsidian graph analyser",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc, graph))),
    )
}
