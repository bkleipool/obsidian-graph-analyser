import os
import re
import json

# Search the target folder and all subfolder for Markdown files
def search_markdown_files(folder_path):
	file_list = []
	title_list = []

	for root, dirs, files in os.walk(folder_path):
		for file in files:
			if file.endswith(".md"):
				file_list.append(os.path.join(root, file)) # Get full file path

				file_title = os.path.splitext(file)[0]  # Get title without extension
				title_list.append(file_title)

	return file_list, title_list


# Search a Markdown file for metadata tags
def search_tags(file):
	with open(file, "r") as f:
		contents = f.read()

		tag_pattern = r"---\n\s*tags: (.*?)\n---"
		match = re.search(tag_pattern, contents, re.DOTALL)

		if match:
			tags_string = match.group(1).strip()
			tags = [tag.strip() for tag in tags_string.split(",")]
		else:
			tags = []

	return tags


# Search a Markdown file for links of the form [[Linked page |...]] or [[Linked page]]
def search_links(file):
	with open(file, "r") as f:
		contents = f.read()

		link_pattern = r"\[\[(.*?)(?:\|.*?)?\]\]" #this is some fuckin magic
		links = re.findall(link_pattern, contents)

	links = list(set([link.strip() for link in links])) # Remove trailing spaces and duplicate results

	return links


vault_dir = "test_vault"
graph = []


# Get files and titles and populate graph
"""
md_files, md_titles = search_markdown_files(vault_dir)

for title in md_titles:
	graph.append({
		"title": title,
		"empty": False,
		"connections": [],
		})

#print(json.dumps(graph, indent=4))

print(search_links(md_files[0]))
"""



md_files, md_titles = search_markdown_files(vault_dir)

print(md_titles)

for file, title in zip(md_files, md_titles):
	tags = search_tags(file)
	links = search_links(file)

	graph.append({
		"title": title,
		"tags": tags,
		"empty": False,
		"links": links,
		})

	for page in links:
		if page not in md_titles:
			graph.append({
			"title": page,
			"tags": [],
			"empty": True,
			"links": [],
			})

print(json.dumps(graph, indent=4))

with open("graph.json", "w") as f:
	f.write(json.dumps(graph, indent=4))