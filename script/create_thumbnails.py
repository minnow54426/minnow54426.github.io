#!/usr/bin/env python3
"""
Create thumbnails for photos
"""

import os
from pathlib import Path
from PIL import Image

def create_thumbnails(photos_dir, thumbs_dir, size=(200, 200)):
    """Create thumbnails for all photos"""
    photos_path = Path(photos_dir)
    thumbs_path = Path(thumbs_dir)
    thumbs_path.mkdir(parents=True, exist_ok=True)

    for photo_file in photos_path.glob("*"):
        if photo_file.suffix.lower() in ['.jpg', '.jpeg', '.png', '.gif']:
            thumb_path = thumbs_path / photo_file.name
            if not thumb_path.exists():
                try:
                    with Image.open(photo_file) as img:
                        img.thumbnail(size, Image.Resampling.LANCZOS)
                        img.save(thumb_path, "JPEG", quality=85, optimize=True)
                    print(f"Created thumbnail: {thumb_path}")
                except Exception as e:
                    print(f"Error creating thumbnail for {photo_file}: {e}")

if __name__ == "__main__":
    create_thumbnails("output/photos", "output/photos/thumbnails")