#!/usr/bin/env python3
import os
from PIL import Image

def generate_thumbnails():
    """Generate thumbnails for all photos in output/photos/"""
    photos_dir = "output/photos"
    thumb_dir = "output/photos/thumbnails"

    # Create thumbnails directory if it doesn't exist
    os.makedirs(thumb_dir, exist_ok=True)

    # Process each photo
    for filename in os.listdir(photos_dir):
        if filename.lower().endswith(('.jpg', '.jpeg', '.png')):
            if not filename.startswith('.') and os.path.isfile(os.path.join(photos_dir, filename)):
                thumb_path = os.path.join(thumb_dir, filename)

                # Skip if thumbnail already exists
                if os.path.exists(thumb_path):
                    print(f"Thumbnail exists: {filename}")
                    continue

                try:
                    # Open image and create thumbnail
                    with Image.open(os.path.join(photos_dir, filename)) as img:
                        # Convert to RGB if necessary
                        if img.mode != 'RGB':
                            img = img.convert('RGB')

                        # Create thumbnail maintaining aspect ratio
                        img.thumbnail((300, 300), Image.Resampling.LANCZOS)

                        # Save thumbnail
                        img.save(thumb_path, 'JPEG', quality=85)
                        print(f"Created thumbnail: {filename}")

                except Exception as e:
                    print(f"Error processing {filename}: {e}")

if __name__ == "__main__":
    generate_thumbnails()
    print("Thumbnail generation complete!")