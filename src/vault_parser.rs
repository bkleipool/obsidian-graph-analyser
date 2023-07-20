use std::{fs, collections::{HashMap, HashSet}, path::{Path, PathBuf}, io::Read};
use petgraph::Graph;
use regex::Regex;

use crate::Page;

// Search the target folder and all subfolders (recursively) for Markdown files
fn search_markdown_files(folder_path: &Path) -> Vec<(PathBuf, String)> {
    let mut file_list = Vec::new();

    fn recursive_file_search(folder_path: &Path, file_list: &mut Vec<(PathBuf, String)>) {
        for entry in fs::read_dir(folder_path).unwrap() {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file() && file_path.extension() == Some(&std::ffi::OsStr::new("md")) {
                    let file_path_buf = file_path.to_path_buf();
                    let file_title = file_path.file_stem().and_then(|os_str| os_str.to_str()).unwrap().to_string();
                    file_list.push((file_path_buf, file_title));
                } else if file_path.is_dir() {
                    recursive_file_search(&file_path, file_list);
                }
            }
        }
    }

    recursive_file_search(folder_path, &mut file_list);
    file_list
}

// Search a Markdown file for metadata tags
fn search_tags(file: &Path) -> Vec<String> {
    let mut contents = String::new();
    fs::File::open(file)
        .and_then(|mut f| f.read_to_string(&mut contents))
        .unwrap_or_default();

    let tag_pattern = Regex::new(r"---\n\s*tags: (.*?)\n---").unwrap();
    if let Some(captures) = tag_pattern.captures(&contents) {
        let tags_string = captures.get(1).unwrap().as_str().trim();
        let tags: Vec<String> = tags_string.split(',').map(|tag| tag.trim().to_string()).collect();
        return tags;
    }
    Vec::new()
}

// Search a Markdown file for links of the form [[Linked page |...]] or [[Linked page]]
fn search_links(file: &Path) -> Vec<String> {
    let mut contents = String::new();
    fs::File::open(file)
        .and_then(|mut f| f.read_to_string(&mut contents))
        .unwrap_or_default();

    let link_pattern = Regex::new(r"\[\[(.*?)(?:\s*\|.*?)?\]\]").unwrap();
    let links: Vec<String> = link_pattern
        .captures_iter(&contents)
        .map(|capture| capture.get(1).map_or("", |m| m.as_str()).trim().to_string())
        .filter(|link| !link.is_empty())
        .collect();
    links.into_iter().collect::<HashSet<_>>().into_iter().collect()
}

// Extract all markdown files from a directory
fn extract_pages(vault_dir: &Path) -> Vec<Page> {
    let mut pages = Vec::new();
    let md_files = search_markdown_files(vault_dir);

    for (file, title) in &md_files {
        let tags = search_tags(file);
        let links = search_links(file);

        pages.push(Page {
            title: title.to_string(),
            tags,
            empty: false,
            links: links.clone(),
        });

        for page in &links {
            if !md_files.iter().any(|(_, t)| t == page) {
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
pub fn vault_to_graph(vault_dir: &Path) -> Graph<Page, ()> {
    pages_to_graph(extract_pages(vault_dir))
}