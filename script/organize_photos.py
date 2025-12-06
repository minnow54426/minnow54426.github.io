#!/usr/bin/env python3
"""
Script to organize photos by year and month based on file modification dates
"""

import os
import shutil
import argparse
from pathlib import Path
from datetime import datetime

def get_photo_date(file_path):
    """
    Get date from file modification time

    Args:
        file_path (Path): Path to the photo file

    Returns:
        datetime: Date of the photo
    """
    # Use file modification date
    mtime = os.path.getmtime(file_path)
    return datetime.fromtimestamp(mtime)

def organize_photos(source_dir, target_dir, move=False):
    """
    Organize photos by year and month

    Args:
        source_dir (str): Directory containing photos to organize
        target_dir (str): Directory to organize photos into
        move (bool): If True, move files; if False, copy files
    """
    source_path = Path(source_dir)
    target_path = Path(target_dir)

    if not source_path.exists():
        print(f"Error: Source directory {source_dir} does not exist")
        return False

    # Create target directory if it doesn't exist
    target_path.mkdir(parents=True, exist_ok=True)

    # Process all image files
    image_extensions = {'.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', '.heic', '.dng', '.arw', '.mov'}
    processed_count = 0

    for file_path in source_path.iterdir():
        if file_path.is_file() and file_path.suffix.lower() in image_extensions:
            # Get photo date
            photo_date = get_photo_date(file_path)

            # Create year/month directory structure
            year_dir = target_path / str(photo_date.year)
            month_dir = year_dir / f"{photo_date.month:02d}"
            month_dir.mkdir(parents=True, exist_ok=True)

            # Copy or move file to organized directory
            target_file = month_dir / file_path.name

            # Skip if file already exists
            if target_file.exists():
                print(f"Skipping {file_path.name} - already exists")
                continue

            if move:
                shutil.move(str(file_path), str(target_file))
                print(f"Moved {file_path.name} to {month_dir}")
            else:
                shutil.copy2(file_path, target_file)
                print(f"Copied {file_path.name} to {month_dir}")

            processed_count += 1

    print(f"Processed {processed_count} photos")
    return True

def main():
    parser = argparse.ArgumentParser(description="Organize photos by year and month")
    parser.add_argument("--source", "-s",
                       default="content/content/photos",
                       help="Source directory containing photos (default: content/content/photos)")
    parser.add_argument("--target", "-t",
                       default="content/content/photos/organized",
                       help="Target directory for organized photos (default: content/content/photos/organized)")
    parser.add_argument("--move", "-m", action="store_true",
                       help="Move files instead of copying")

    args = parser.parse_args()

    # Resolve relative paths from script directory
    script_dir = Path(__file__).parent
    source_dir = script_dir.parent / args.source
    target_dir = script_dir.parent / args.target

    print(f"Organizing photos from {source_dir} to {target_dir}...")
    if organize_photos(str(source_dir), str(target_dir), args.move):
        print("Photo organization complete!")
    else:
        print("Photo organization failed!")
        exit(1)

if __name__ == "__main__":
    main()