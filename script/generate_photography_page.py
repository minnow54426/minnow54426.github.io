#!/usr/bin/env python3
"""
Generate photography page with all photos
"""

import os
import glob
from PIL import Image

def generate_thumbnails():
    """Generate thumbnails for photos"""
    photos_dir = "output/photos"
    thumb_dir = "output/photos/thumbnails"

    os.makedirs(thumb_dir, exist_ok=True)

    for filename in os.listdir(photos_dir):
        if filename.lower().endswith(('.jpg', '.jpeg', '.png')):
            if not filename.startswith('.') and os.path.isfile(os.path.join(photos_dir, filename)):
                thumb_path = os.path.join(thumb_dir, filename)

                # Skip if thumbnail already exists
                if os.path.exists(thumb_path):
                    continue

                try:
                    # Open image and create thumbnail
                    img_path = os.path.join(photos_dir, filename)
                    with Image.open(img_path) as img:
                        # Convert to RGB if necessary
                        if img.mode != 'RGB':
                            img = img.convert('RGB')

                        # Create thumbnail
                        img.thumbnail((300, 300), Image.Resampling.LANCZOS)

                        # Save thumbnail
                        img.save(thumb_path, 'JPEG', quality=85)
                        print(f"Created thumbnail: {filename}")

                except Exception as e:
                    print(f"Error processing {filename}: {e}")

def generate_photography_page():
    photos_dir = "content/photos"
    output_photos_dir = "output/photos"
    output_file = "output/photography.html"

    # Create output photos directory
    os.makedirs(output_photos_dir, exist_ok=True)

    # Copy photos
    for ext in ["*.jpg", "*.JPG", "*.jpeg", "*.JPEG", "*.png", "*.PNG"]:
        for photo in glob.glob(os.path.join(photos_dir, ext)):
            if not os.path.isfile(photo) or photo.startswith('.'):
                continue
            os.system(f'cp "{photo}" "{output_photos_dir}/"')

    # Generate thumbnails
    generate_thumbnails()

    # Get all photo files
    photos = [f for f in os.listdir(output_photos_dir)
              if f.lower().endswith(('.jpg', '.jpeg', '.png', '.gif'))
              and not f.startswith('thumbnail')]

    # Sort photos
    photos.sort()

    # Generate JavaScript array
    js_photos = ',\n            '.join([f"'{photo}'" for photo in photos])

    # Read template
    with open(output_file, 'r') as f:
        content = f.read()

    # Replace the photos array
    import re
    pattern = r'const photos = \[.*?\];'
    replacement = f'const photos = [\n            {js_photos}\n        ];'
    content = re.sub(pattern, replacement, content, flags=re.DOTALL)

    # Write back
    with open(output_file, 'w') as f:
        f.write(content)

    print(f"Updated photography page with {len(photos)} photos")

if __name__ == "__main__":
    generate_photography_page()