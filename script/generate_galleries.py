#!/usr/bin/env python3
"""
Simplified photo gallery generator
Creates thumbnails and galleries from manually categorized photos
"""

import os
import argparse
from pathlib import Path
from datetime import datetime
from PIL import Image

def create_thumbnail(photo_path, thumbnail_path, size=(300, 300)):
    """Create a thumbnail of the given photo"""
    try:
        with Image.open(photo_path) as img:
            img.thumbnail(size, Image.Resampling.LANCZOS)
            img.save(thumbnail_path, "JPEG", quality=85, optimize=True)
        return True
    except Exception as e:
        print(f"Error creating thumbnail for {photo_path}: {e}")
        return False

def generate_galleries(categories_dir, output_dir):
    """Generate gallery pages for all categories"""
    categories_path = Path(categories_dir)
    output_path = Path(output_dir)

    # Create thumbnails directory
    thumbnails_dir = output_path / "thumbnails"
    thumbnails_dir.mkdir(parents=True, exist_ok=True)

    # Process each category
    for category_dir in categories_path.iterdir():
        if category_dir.is_dir():
            category_name = category_dir.name
            photo_files = [f for f in os.listdir(category_dir)
                          if f.lower().endswith(('.jpg', '.jpeg', '.png', '.gif'))]

            if not photo_files:
                continue

            # Create gallery markdown
            current_date = datetime.now().strftime("%Y-%m-%d")
            category_display = category_name.replace('-', ' ').title()

            content = f"""Title: {category_display} Photos
Date: {current_date}
Category: Photography
Tags: photos, gallery, {category_name}
Slug: {category_name}-gallery
Author: Cryptboy
Summary: {category_display} photo collection

{category_display} photographs.

<!-- PELICAN_END_SUMMARY -->

<div class="photo-gallery">
"""

            # Add photos
            for photo in sorted(photo_files):
                photo_path = category_dir / photo
                thumbnail_path = thumbnails_dir / photo

                # Create thumbnail if needed
                if not thumbnail_path.exists():
                    create_thumbnail(photo_path, thumbnail_path)

                # URLs
                thumbnail_url = f"/photography/thumbnails/{photo}"
                photo_url = f"/photos/manual-categories/{category_name}/{photo}"

                content += f"""  <div class="photo-item">
    <a href="{photo_url}" class="lightbox-link">
      <img src="{thumbnail_url}" alt="{category_display}" />
    </a>
  </div>
"""

            content += """</div>

<link rel="stylesheet" href="/theme/css/gallery.css" />
<script src="/theme/js/gallery.js"></script>
"""

            # Write gallery file
            gallery_file = output_path / f"{category_name}.md"
            with open(gallery_file, "w") as f:
                f.write(content)

            print(f"Generated gallery: {gallery_file}")

def generate_folder_indexes(categories_dir, output_dir):
    """Generate HTML index pages for folder browsing"""
    categories_path = Path(categories_dir)
    output_path = Path(output_dir)

    # Main index
    index_content = """<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Photo Categories</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .category-list { list-style: none; padding: 0; }
        .category-list li { margin: 10px 0; }
        .category-list a { text-decoration: none; color: #0066cc; font-size: 18px; }
        .category-list a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <h1>Photo Categories</h1>
    <ul class="category-list">
"""

    # Generate category pages
    for category_dir in categories_path.iterdir():
        if category_dir.is_dir():
            category_name = category_dir.name
            category_display = category_name.replace('-', ' ').title()

            # Add to main index
            index_content += f'        <li><a href="{category_name}/">{category_display}</a></li>\n'

            # Category page
            photo_files = [f for f in os.listdir(category_dir)
                          if f.lower().endswith(('.jpg', '.jpeg', '.png', '.gif'))]

            category_content = f"""<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{category_display} Photos</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .back-link {{ margin-bottom: 20px; }}
        .photo-grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 10px; }}
        .photo-grid img {{ width: 100%; height: auto; border-radius: 4px; }}
    </style>
</head>
<body>
    <div class="back-link"><a href="../">← Back to Categories</a></div>
    <h1>{category_display} Photos</h1>
    <div class="photo-grid">
"""

            for photo in sorted(photo_files):
                category_content += f'        <img src="{photo}" alt="{category_display}" />\n'

            category_content += """    </div>
</body>
</html>"""

            # Write category page
            cat_output_dir = output_path / category_name
            cat_output_dir.mkdir(parents=True, exist_ok=True)
            with open(cat_output_dir / "index.html", "w") as f:
                f.write(category_content)

    index_content += """    </ul>
</body>
</html>"""

    # Write main index
    output_path.mkdir(parents=True, exist_ok=True)
    with open(output_path / "index.html", "w") as f:
        f.write(index_content)

    print(f"Generated folder indexes in {output_path}")

def main():
    parser = argparse.ArgumentParser(description='Generate photo galleries')
    parser.add_argument('--categories-dir', default='content/content/photos/manual-categories',
                       help='Directory with categorized photos')
    parser.add_argument('--output-dir', default='content/content/photography',
                       help='Output directory for galleries')
    parser.add_argument('--folder-indexes', action='store_true',
                       help='Generate HTML folder indexes')
    parser.add_argument('--indexes-output', default='output/photos/manual-categories',
                       help='Output for HTML indexes')

    args = parser.parse_args()

    # Generate galleries
    generate_galleries(args.categories_dir, args.output_dir)

    # Generate folder indexes if requested
    if args.folder_indexes:
        generate_folder_indexes(args.categories_dir, args.indexes_output)

if __name__ == "__main__":
    main()