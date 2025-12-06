#!/usr/bin/env python3
"""
Script to generate sub-gallery pages for each year/month
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
            # Calculate thumbnail size maintaining aspect ratio
            img.thumbnail(size, Image.Resampling.LANCZOS)
            img.save(thumbnail_path, "JPEG", quality=85, optimize=True)
        return True
    except Exception as e:
        print(f"Error creating thumbnail for {photo_path}: {e}")
        return False

def generate_gallery_page(year, month, photo_files, output_dir, base_url=""):
    """
    Generate a gallery page for a specific year/month

    Args:
        year (str): Year (e.g., "2025")
        month (str): Month (e.g., "10")
        photo_files (list): List of photo filenames
        output_dir (str): Directory to save the gallery page
        base_url (str): Base URL for photo links
    """
    # Month names for better readability
    month_names = {
        "01": "January", "02": "February", "03": "March", "04": "April",
        "05": "May", "06": "June", "07": "July", "08": "August",
        "09": "September", "10": "October", "11": "November", "12": "December"
    }

    month_name = month_names.get(month, month)

    # Create thumbnails directory
    thumbnails_dir = Path(output_dir) / f"{year}" / f"{month}" / "thumbnails"
    thumbnails_dir.mkdir(parents=True, exist_ok=True)

    # Create the markdown content with current date
    current_date = datetime.now().strftime("%Y-%m-%d")
    content = f"""Title: {month_name} {year} Photo Gallery
Date: {current_date}
Category: Photography
Tags: photos, gallery, {year}, {month_name.lower()}
Slug: {year}/{month}
Author: Cryptboy
Summary: Photos from {month_name} {year}

Photos from {month_name} {year}.

<!-- PELICAN_END_SUMMARY -->

<div class="photo-gallery">
"""

    # Add each photo to the gallery
    for photo in photo_files:
        # Skip non-image files for display
        if photo.lower().endswith(('.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', '.heic', '.dng', '.arw')):
            # Create thumbnail
            photo_path = Path(output_dir).parent.parent / "photos" / "organized" / f"{year}" / f"{month}" / photo
            thumbnail_path = thumbnails_dir / photo

            # Create thumbnail if it doesn't exist
            if not thumbnail_path.exists() and photo_path.exists():
                create_thumbnail(photo_path, thumbnail_path, (300, 300))

            # URLs for web
            thumbnail_url = f"/photography/{year}/{month}/thumbnails/{photo}"
            photo_url = f"/photos/organized/{year}/{month}/{photo}"

            # Add photo to gallery with lightbox functionality
            content += f"""  <div class="photo-item">
    <a href="{photo_url}" class="lightbox-link" data-lightbox="gallery">
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
    output_path = Path(output_dir) / f"{year}" / f"{month}"
    output_path.mkdir(parents=True, exist_ok=True)

    gallery_file = output_path / "index.md"
    with open(gallery_file, "w") as f:
        f.write(content)

    print(f"Generated gallery page for {year}/{month} with {len(photo_files)} photos")

def generate_all_galleries(organized_photos_dir, output_dir):
    """
    Generate gallery pages for all year/month directories

    Args:
        organized_photos_dir (str): Directory with organized photos (year/month structure)
        output_dir (str): Directory to save gallery pages
    """
    organized_path = Path(organized_photos_dir)

    if not organized_path.exists():
        print(f"Error: Organized photos directory {organized_photos_dir} does not exist")
        return False

    generated_count = 0

    # Iterate through years
    for year_dir in organized_path.iterdir():
        if year_dir.is_dir():
            year = year_dir.name

            # Iterate through months
            for month_dir in year_dir.iterdir():
                if month_dir.is_dir():
                    month = month_dir.name

                    # Get all photos in this month
                    photo_files = [f.name for f in month_dir.iterdir() if f.is_file()]

                    # Generate gallery page
                    if photo_files:
                        generate_gallery_page(year, month, photo_files, output_dir)
                        generated_count += 1

    print(f"Generated {generated_count} gallery pages")
    return True

def main():
    parser = argparse.ArgumentParser(description="Generate gallery pages from organized photos")
    parser.add_argument("--photos-dir", "-p",
                       default="content/content/photos/organized",
                       help="Directory with organized photos (default: content/content/photos/organized)")
    parser.add_argument("--output-dir", "-o",
                       default="content/content/photography",
                       help="Output directory for gallery pages (default: content/content/photography)")

    args = parser.parse_args()

    # Resolve relative paths from script directory
    script_dir = Path(__file__).parent
    photos_dir = script_dir.parent / args.photos_dir
    output_dir = script_dir.parent / args.output_dir

    print(f"Generating galleries from {photos_dir} to {output_dir}...")
    if generate_all_galleries(str(photos_dir), str(output_dir)):
        print("Gallery page generation complete!")
    else:
        print("Gallery page generation failed!")
        exit(1)

if __name__ == "__main__":
    main()