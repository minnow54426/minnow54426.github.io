#!/usr/bin/env python3
"""
Script to generate gallery pages from manually categorized photos
"""

import os
import argparse
from pathlib import Path
from datetime import datetime
from PIL import Image

def create_thumbnail(photo_path, thumbnail_path, size=(300, 300)):
    """
    Create a thumbnail of the given photo

    Args:
        photo_path (str): Path to the original photo
        thumbnail_path (str): Path where thumbnail should be saved
        size (tuple): Maximum size for the thumbnail (width, height)
    """
    try:
        with Image.open(photo_path) as img:
            img.thumbnail(size, Image.Resampling.LANCZOS)
            img.save(thumbnail_path, "JPEG", quality=85, optimize=True)
        return True
    except Exception as e:
        print(f"Error creating thumbnail for {photo_path}: {e}")
        return False

def generate_category_gallery(category_name, photo_files, output_dir, photos_base_dir):
    """
    Generate a gallery page for a specific category

    Args:
        category_name (str): Name of the category
        photo_files (list): List of photo filenames
        output_dir (str): Directory to save the gallery page
        photos_base_dir (str): Base directory where photos are stored
    """
    # Create thumbnails directory
    thumbnails_dir = Path(output_dir) / "thumbnails"
    thumbnails_dir.mkdir(parents=True, exist_ok=True)

    # Create the markdown content
    current_date = datetime.now().strftime("%Y-%m-%d")
    category_display = category_name.replace('-', ' ').title()

    content = f"""Title: {category_display} Photos
Date: {current_date}
Category: Photography
Tags: photos, gallery, {category_name}
Slug: {category_name}
Author: Cryptboy
Summary: {category_display} photo collection

A collection of {category_display} photographs.

<!-- PELICAN_END_SUMMARY -->

<div class="photo-gallery">
"""

    # Add each photo to the gallery
    for photo in photo_files:
        if photo.lower().endswith(('.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', '.heic', '.dng', '.arw')):
            # Create thumbnail
            photo_path = Path(photos_base_dir) / category_name / photo
            thumbnail_path = thumbnails_dir / photo

            # Create thumbnail if it doesn't exist
            if not thumbnail_path.exists() and photo_path.exists():
                create_thumbnail(photo_path, thumbnail_path, (300, 300))

            # URLs for web
            thumbnail_url = f"/photography/thumbnails/{photo}"
            photo_url = f"/photos/manual-categories/{category_name}/{photo}"

            # Add photo to gallery
            content += f"""  <div class="photo-item">
    <a href="{photo_url}" class="lightbox-link" data-lightbox="{category_name}">
      <img src="{thumbnail_url}" alt="{photo}" />
    </a>
  </div>
"""

    # Reference external CSS and JS files
    content += """</div>

<link rel="stylesheet" href="/theme/css/gallery.css" />
<script src="/theme/js/gallery.js"></script>
"""

    # Write the file
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)

    gallery_file = output_path / "index.md"
    with open(gallery_file, "w") as f:
        f.write(content)

    print(f"Generated {category_name} gallery with {len(photo_files)} photos")

def generate_category_index(categories_data, output_dir):
    """
    Generate an index page listing all photo categories

    Args:
        categories_data (dict): Dictionary of category names and photo counts
        output_dir (str): Directory to save the index page
    """
    current_date = datetime.now().strftime("%Y-%m-%d")

    content = f"""Title: Photo Categories
Date: {current_date}
Category: Photography
Tags: photos, categories
Slug: photo-categories
Author: Cryptboy
Summary: Browse photos by category

Browse my photo collection by category.

<!-- PELICAN_END_SUMMARY -->

<div class="category-grid">
"""

    # Sort categories by name
    for category_name, photo_count in sorted(categories_data.items()):
        category_display = category_name.replace('-', ' ').title()
        category_url = f"/photography/{category_name}/"

        # Try to find a representative image for the category
        sample_image = ""
        category_dir = Path(output_dir).parent / "photos" / "manual-categories" / category_name
        if category_dir.exists():
            images = [f for f in category_dir.iterdir()
                     if f.suffix.lower() in {'.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', '.heic'}]
            if images:
                sample_image = f'<img src="/photos/manual-categories/{category_name}/{images[0].name}" alt="{category_display}" />'

        content += f"""  <div class="category-card">
    <a href="{category_url}" class="category-link">
      {sample_image}
      <div class="category-info">
        <h3>{category_display}</h3>
        <span class="photo-count">{photo_count} photos</span>
      </div>
    </a>
  </div>
"""

    content += """</div>

<style>
  .category-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 20px;
    margin-top: 20px;
  }

  .category-card {
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s ease, box-shadow 0.3s ease;
  }

  .category-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.15);
  }

  .category-link {
    display: block;
    text-decoration: none;
    color: inherit;
  }

  .category-link img {
    width: 100%;
    height: 200px;
    object-fit: cover;
    display: block;
  }

  .category-info {
    padding: 15px;
    background: white;
  }

  .category-info h3 {
    margin: 0 0 5px 0;
    font-size: 1.2em;
  }

  .photo-count {
    color: #666;
    font-size: 0.9em;
  }
</style>
"""

    # Write the file
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)

    index_file = output_path / "photo-categories.md"
    with open(index_file, "w") as f:
        f.write(content)

    print(f"Generated category index with {len(categories_data)} categories")

def generate_all_category_galleries(categories_dir, photos_dir, output_dir):
    """
    Generate gallery pages for all category folders

    Args:
        categories_dir (str): Directory containing category folders
        photos_dir (str): Directory where photos are stored
        output_dir (str): Directory to save gallery pages
    """
    categories_path = Path(categories_dir)

    if not categories_path.exists():
        print(f"Categories directory {categories_dir} does not exist")
        return False

    categories_data = {}
    generated_count = 0

    # Iterate through category directories
    for category_dir in categories_path.iterdir():
        if category_dir.is_dir():
            category_name = category_dir.name

            # Get all photos in this category
            photo_files = [f.name for f in category_dir.iterdir() if f.is_file()]

            if photo_files:
                # Generate category gallery
                category_output_dir = Path(output_dir) / category_name
                generate_category_gallery(category_name, photo_files, str(category_output_dir), photos_dir)
                categories_data[category_name] = len(photo_files)
                generated_count += 1

    # Generate category index
    if categories_data:
        generate_category_index(categories_data, output_dir)

    print(f"\nGenerated {generated_count} category galleries")
    return True

def main():
    parser = argparse.ArgumentParser(description="Generate category-based photo galleries")
    parser.add_argument("--categories-dir", "-c",
                       default="content/photos/manual-categories",
                       help="Directory with category folders (default: content/photos/manual-categories)")
    parser.add_argument("--photos-dir", "-p",
                       default="content/photos",
                       help="Base directory for photos (default: content/photos)")
    parser.add_argument("--output-dir", "-o",
                       default="content/content/photography",
                       help="Output directory for gallery pages (default: content/content/photography)")

    args = parser.parse_args()

    # Resolve relative paths from script directory
    script_dir = Path(__file__).parent
    categories_dir = script_dir.parent / args.categories_dir
    photos_dir = script_dir.parent / args.photos_dir
    output_dir = script_dir.parent / args.output_dir

    print(f"Generating category galleries from {categories_dir}")
    if generate_all_category_galleries(str(categories_dir), str(photos_dir), str(output_dir)):
        print("Category gallery generation complete!")
    else:
        print("Category gallery generation failed!")
        exit(1)

if __name__ == "__main__":
    main()