#!/usr/bin/env python3
"""
Script to generate photo gallery HTML
"""

import os
from pathlib import Path

def generate_photo_gallery_html():
    """Generate HTML for photo gallery"""
    # Get the current working directory
    cwd = Path.cwd()
    photos_dir = cwd / "content" / "photos"
    
    print(f"Looking for photos in: {photos_dir}")
    
    if not photos_dir.exists():
        print("Photos directory not found")
        return ""
    
    photos = []
    for file in photos_dir.iterdir():
        if file.is_file() and file.suffix.lower() in ['.png', '.jpg', '.jpeg', '.gif', '.bmp', '.tiff', '.webp', '.heic']:
            photos.append(file.name)
    
    photos.sort()
    
    if not photos:
        return "<p>No photos found.</p>"
    
    # Generate HTML for photo gallery
    html = '<div class="photo-gallery">\n'
    for photo in photos:
        html += f'  <div class="photo-item">\n'
        html += f'    <img src="/photos/{photo}" alt="{photo}" />\n'
        html += f'  </div>\n'
    html += '</div>\n'
    
    return html

if __name__ == "__main__":
    html = generate_photo_gallery_html()
    print(html)