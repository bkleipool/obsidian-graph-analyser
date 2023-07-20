use std::{fs, collections::HashMap};
use petgraph::Graph;
use regex::Regex;

use crate::Page;

// Search the target folder and all subfolders for Markdown files
fn search_markdown_files(folder_path: &str) -> (Vec<String>, Vec<String>) {
    let mut file_list = Vec::new();
    let mut title_list = Vec::new();

    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "md" {
            file_list.push(path.to_string_lossy().to_string());
            title_list.push(path.file_stem().unwrap().to_string_lossy().to_string());
        }
    }

    (file_list, title_list)
}

// Search a Markdown file for metadata tags
fn search_tags(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).unwrap();
    let tag_pattern = r"---\n\s*tags: (.*?)\n---";
    let re = Regex::new(tag_pattern).unwrap();

    if let Some(captures) = re.captures(&contents) {
        let tags_string = captures.get(1).unwrap().as_str().trim();
        let tags = tags_string.split(',').map(|tag| tag.trim().to_string()).collect();
        tags
    } else {
        Vec::new()
    }
}

// Search a Markdown file for links of the form [[Linked page |...]] or [[Linked page]]
fn search_links(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).unwrap();
    let link_pattern = r"\[\[(.*?)(?:\|.*?)?\]\]";
    let re = Regex::new(link_pattern).unwrap();

    let links: Vec<String> = re.captures_iter(&contents)
        .map(|capture| capture.get(1).unwrap().as_str().trim().to_string())
        .collect();

    links.into_iter().collect()
}

fn extract_pages(vault_dir: &str) -> Vec<Page> {
    let mut pages = Vec::new();

    let (md_files, md_titles) = search_markdown_files(vault_dir);

    for (file, title) in md_files.iter().zip(md_titles.iter()) {
        let tags = search_tags(file);
        let links = search_links(file);

        pages.push(Page {
            title: title.to_string(),
            tags,
            empty: false,
            links: links.clone(),
        });

        for page in &links {
            if !md_titles.contains(&page) {
                pages.push(Page {
                    title: page.to_string(),
                    tags: Vec::new(),
                    empty: true,
                    links: Vec::new(),
                });
            }
        }
    }

    pages
}


// Reads a vector of Page structs and converts it to a petgraph instance
fn pages_to_graph(pages: Vec<Page>) -> Graph<Page, ()> {
    // Create a directed graph
    let mut graph: Graph<Page, ()> = Graph::new();

    // Create a hashmap to quickly find nodes (pages) by their title
    let mut title_to_node = HashMap::new();

    // Add nodes (pages) to the graph and populate the hashmap
    for page in &pages {
        let page_index = *title_to_node.entry(page.title.clone()).or_insert_with(|| {
            let new_page_index = graph.add_node(page.clone());
            new_page_index
        });
        title_to_node.insert(page.title.clone(), page_index);
    }

    // Add edges (links) to the graph
    for page in pages.iter() {
        let source_node_index = title_to_node.get(&page.title).expect("Node not found");
        for linked_page_title in page.links.iter() {
            let target_node_index = title_to_node
                .get(linked_page_title)
                .expect("Node not found");
            graph.add_edge(*source_node_index, *target_node_index, ());
        }
    }

    graph
}


/// Converts an Obsidian vault to a petgraph instance
pub fn vault_to_graph(vault_dir: &str) -> Graph<Page, ()> {
    pages_to_graph(extract_pages(vault_dir))
}