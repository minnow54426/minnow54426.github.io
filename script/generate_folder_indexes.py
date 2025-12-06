#!/usr/bin/env python3
"""
Script to generate index.html files for photo folders
This creates simple gallery pages for each category folder
"""

import os
import argparse
from pathlib import Path
from datetime import datetime

def generate_folder_index(category_name, photos_dir, output_dir):
    """
    Generate an index.html for a photo folder

    Args:
        category_name (str): Name of the category
        photos_dir (str): Directory where photos are stored
        output_dir (str): Output directory for the index file
    """
    category_path = Path(photos_dir) / category_name
    output_path = Path(output_dir) / category_name

    if not category_path.exists():
        print(f"Category directory {category_path} does not exist")
        return False

    # Get all photos in the category
    photo_files = [f for f in category_path.iterdir()
                  if f.is_file() and f.suffix.lower() in
                  {'.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', '.heic', '.dng', '.arw', '.mov'}]

    # Sort photos
    photo_files.sort()

    # Create HTML content
    category_display = category_name.replace('-', ' ').title()

    html = f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{category_display} Photos - wonderonpathlesspath</title>
    <link rel="stylesheet" href="/theme/css/gallery.css">
    <style>
        .breadcrumb {{
            margin: 20px 0;
            color: #666;
        }}
        .breadcrumb a {{
            color: #0066cc;
            text-decoration: none;
        }}
        .breadcrumb a:hover {{
            text-decoration: underline;
        }}
        .folder-info {{
            background: #f5f5f5;
            padding: 15px;
            border-radius: 8px;
            margin-bottom: 20px;
        }}
        .back-link {{
            display: inline-block;
            margin-bottom: 20px;
            color: #0066cc;
            text-decoration: none;
        }}
        .back-link:hover {{
            text-decoration: underline;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1><a href="/">wonderonpathlesspath</a></h1>
            <nav>
                <ul>
                    <li><a href="/">Home</a></li>
                    <li><a href="/photography/photo-categories.html">Photo Categories</a></li>
                    <li><a href="/photos/manual-categories/">Photo Folders</a></li>
                </ul>
            </nav>
        </header>

        <main>
            <div class="breadcrumb">
                <a href="/">Home</a> / <a href="/photography/">Photography</a> / <a href="/photos/manual-categories/">Photo Folders</a> / {category_display}
            </div>

            <a href="/photos/manual-categories/" class="back-link">← Back to Photo Folders</a>

            <h1>{category_display} Photos</h1>

            <div class="folder-info">
                <p><strong>Category:</strong> {category_display}</p>
                <p><strong>Photos:</strong> {len(photo_files)} images</p>
                <p><strong>Folder:</strong> /photos/manual-categories/{category_name}/</p>
            </div>

            <div class="photo-gallery">
"""

    # Add each photo to the gallery
    for photo in photo_files:
        photo_url = f"/photos/manual-categories/{category_name}/{photo.name}"
        html += f"""                <div class="photo-item">
                    <a href="{photo_url}" class="lightbox-link" data-lightbox="{category_name}">
                        <img src="{photo_url}" alt="{photo.name}" loading="lazy" />
                    </a>
                </div>
"""

    # Close HTML
    html += """            </div>

            <script src="/theme/js/gallery.js"></script>
        </main>

        <footer>
            <p>&copy; """ + str(datetime.now().year) + """ Cryptboy</p>
        </footer>
    </div>
</body>
</html>"""

    # Write the file
    output_path.mkdir(parents=True, exist_ok=True)
    index_file = output_path / "index.html"

    with open(index_file, "w") as f:
        f.write(html)

    print(f"Generated index for {category_name} with {len(photo_files)} photos")
    return True

def generate_all_folder_indexes(categories_dir, output_dir):
    """
    Generate index.html for all category folders

    Args:
        categories_dir (str): Base directory for categories
        output_dir (str): Output directory for index files
    """
    categories_path = Path(categories_dir)

    if not categories_path.exists():
        print(f"Categories directory {categories_dir} does not exist")
        return False

    generated_count = 0

    for category_dir in categories_path.iterdir():
        if category_dir.is_dir():
            if generate_folder_index(category_dir.name, categories_dir, output_dir):
                generated_count += 1

    print(f"\nGenerated {generated_count} folder indexes")
    return True

def main():
    parser = argparse.ArgumentParser(description="Generate index files for photo folders")
    parser.add_argument("--categories-dir", "-c",
                       default="content/photos/manual-categories",
                       help="Directory with category folders")
    parser.add_argument("--output-dir", "-o",
                       default="output/photos/manual-categories",
                       help="Output directory for index files")

    args = parser.parse_args()

    # Resolve relative paths
    script_dir = Path(__file__).parent
    categories_dir = script_dir.parent / args.categories_dir
    output_dir = script_dir.parent / args.output_dir

    print(f"Generating folder indexes from {categories_dir}")
    if generate_all_folder_indexes(str(categories_dir), str(output_dir)):
        print("Folder index generation complete!")
    else:
        print("Folder index generation failed!")
        exit(1)

if __name__ == "__main__":
    main()