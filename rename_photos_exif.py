#!/usr/bin/env python3
"""
Rename photos based on their actual EXIF capture date
Format: YYYY-MM-DD-001.jpg (sequential within each category)
"""

from PIL import Image
from PIL.ExifTags import TAGS
import os
import sys
from datetime import datetime

def get_exif_date(image_path):
    """Extract the date/time from EXIF data"""
    try:
        image = Image.open(image_path)
        exif = image._getexif()

        if exif is None:
            return None

        # EXIF tag 36867 is DateTimeOriginal
        # EXIF tag 306 is DateTime
        for tag_id in [36867, 306, 36868]:
            if tag_id in exif:
                date_str = exif[tag_id]
                # Format: "2024:08:22 21:02:19"
                try:
                    dt = datetime.strptime(date_str, "%Y:%m:%d %H:%M:%S")
                    return dt
                except ValueError:
                    continue

        return None
    except Exception as e:
        print(f"  Error reading {image_path}: {e}", file=sys.stderr)
        return None

def get_file_date(image_path):
    """Fallback to file modification date"""
    try:
        timestamp = os.path.getmtime(image_path)
        return datetime.fromtimestamp(timestamp)
    except Exception as e:
        print(f"  Error getting file date: {e}", file=sys.stderr)
        return datetime.now()

def get_photo_date(image_path):
    """Get photo date from EXIF or file modification"""
    exif_date = get_exif_date(image_path)
    if exif_date:
        return exif_date
    return get_file_date(image_path)

def process_category(category_dir):
    """Process all photos in a category directory"""
    category = os.path.basename(category_dir)
    print(f"Processing: {category}")

    # Get all image files
    image_files = []
    for filename in os.listdir(category_dir):
        if filename.lower().endswith(('.jpg', '.jpeg', '.png')):
            image_files.append(os.path.join(category_dir, filename))

    # Sort by original date
    image_files_with_dates = []
    for img_path in image_files:
        photo_date = get_photo_date(img_path)
        image_files_with_dates.append((photo_date, img_path))

    # Sort by date
    image_files_with_dates.sort(key=lambda x: x[0])

    # Rename with counter
    counter = 1
    for photo_date, old_path in image_files_with_dates:
        # Get file extension
        ext = os.path.splitext(old_path)[1].lower()
        if not ext:
            ext = '.jpg'

        # Format date as YYYY-MM-DD
        date_str = photo_date.strftime("%Y-%m-%d")

        # Create new filename
        new_filename = f"{date_str}-{counter:03d}{ext}"
        new_path = os.path.join(category_dir, new_filename)

        # Rename
        try:
            os.rename(old_path, new_path)
            old_name = os.path.basename(old_path)
            capture_date = photo_date.strftime("%Y-%m-%d %H:%M")
            print(f"  {old_name} ({capture_date}) → {new_filename}")
        except Exception as e:
            print(f"  Error renaming {old_path}: {e}", file=sys.stderr)

        counter += 1

    print()
    return counter - 1  # Return count of renamed files

def main():
    print("=== Photo Renaming Script (EXIF Date Based) ===")
    print("Extracts actual capture date from EXIF metadata")
    print("Format: YYYY-MM-DD-001.jpg (sorted by capture date within each category)")
    print("")

    photos_dir = "photos"
    if not os.path.isdir(photos_dir):
        print(f"Error: {photos_dir} directory not found")
        sys.exit(1)

    # Get all category directories
    categories = [d for d in os.listdir(photos_dir)
                  if os.path.isdir(os.path.join(photos_dir, d))]
    categories.sort()

    total_renamed = 0
    for category in categories:
        category_path = os.path.join(photos_dir, category)
        count = process_category(category_path)
        total_renamed += count

    print(f"=== Complete ===")
    print(f"Total photos renamed: {total_renamed}")

if __name__ == "__main__":
    main()
