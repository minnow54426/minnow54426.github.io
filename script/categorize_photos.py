#!/usr/bin/env python3
"""
Script to help manually categorize photos into folders
This script provides utilities for managing photo categories
"""

import os
import shutil
import argparse
from pathlib import Path
from datetime import datetime

def create_category_structure(base_dir, categories):
    """
    Create category folder structure

    Args:
        base_dir (str): Base directory for categories
        categories (list): List of category names
    """
    base_path = Path(base_dir)
    base_path.mkdir(parents=True, exist_ok=True)

    for category in categories:
        category_path = base_path / category
        category_path.mkdir(exist_ok=True)
        print(f"Created category folder: {category_path}")

def list_categories(base_dir):
    """
    List all category folders and their photo counts

    Args:
        base_dir (str): Base directory for categories
    """
    base_path = Path(base_dir)

    if not base_path.exists():
        print(f"Category directory {base_dir} does not exist")
        return

    print("\nPhoto Categories:")
    print("-" * 40)

    total_photos = 0
    for category_dir in sorted(base_path.iterdir()):
        if category_dir.is_dir():
            photo_count = len([f for f in category_dir.iterdir()
                             if f.is_file() and f.suffix.lower() in
                             {'.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', '.heic', '.dng', '.arw', '.mov'}])
            print(f"{category_dir.name:15} : {photo_count:3} photos")
            total_photos += photo_count

    print("-" * 40)
    print(f"{'Total':15} : {total_photos:3} photos")

def move_photo_to_category(photo_path, category_dir, copy=False):
    """
    Move or copy a photo to a category folder

    Args:
        photo_path (str): Path to the photo
        category_dir (str): Target category directory
        copy (bool): If True, copy instead of move
    """
    photo = Path(photo_path)
    category = Path(category_dir)

    if not photo.exists():
        print(f"Photo {photo_path} does not exist")
        return False

    category.mkdir(parents=True, exist_ok=True)
    target = category / photo.name

    if target.exists():
        print(f"Photo {photo.name} already exists in {category.name}")
        return False

    if copy:
        shutil.copy2(photo, target)
        print(f"Copied {photo.name} to {category.name}")
    else:
        shutil.move(str(photo), str(target))
        print(f"Moved {photo.name} to {category.name}")

    return True

def batch_categorize(source_dir, category_dir):
    """
    Interactive batch categorization of photos

    Args:
        source_dir (str): Directory with uncategorized photos
        category_dir (str): Base directory for categories
    """
    source = Path(source_dir)
    categories = Path(category_dir)

    if not source.exists():
        print(f"Source directory {source_dir} does not exist")
        return

    # Get available categories
    available_categories = [d.name for d in categories.iterdir() if d.is_dir()]

    if not available_categories:
        print("No category folders found. Please create them first.")
        return

    # Get photos to categorize
    photos = [f for f in source.iterdir()
              if f.is_file() and f.suffix.lower() in
              {'.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', '.heic', '.dng', '.arw', '.mov'}]

    if not photos:
        print("No photos found to categorize")
        return

    print(f"\nFound {len(photos)} photos to categorize")
    print(f"Available categories: {', '.join(available_categories)}")
    print("\nCommands:")
    print("  <category_name> - Move photo to category")
    print("  copy <category_name> - Copy photo to category")
    print("  skip - Skip this photo")
    print("  quit - Exit\n")

    for i, photo in enumerate(photos):
        print(f"\nPhoto {i+1}/{len(photos)}: {photo.name}")

        # Show available categories with numbers
        for j, cat in enumerate(available_categories, 1):
            print(f"  {j}. {cat}")

        while True:
            try:
                cmd = input("Enter command: ").strip().lower()

                if cmd == 'quit':
                    print("Stopping categorization")
                    return
                elif cmd == 'skip':
                    print(f"Skipped {photo.name}")
                    break
                elif cmd.startswith('copy '):
                    cat = cmd[5:]
                    if cat in available_categories:
                        if move_photo_to_category(photo, categories / cat, copy=True):
                            break
                    else:
                        print(f"Category '{cat}' not found")
                elif cmd.isdigit():
                    idx = int(cmd) - 1
                    if 0 <= idx < len(available_categories):
                        cat = available_categories[idx]
                        if move_photo_to_category(photo, categories / cat):
                            break
                    else:
                        print("Invalid category number")
                elif cmd in available_categories:
                    if move_photo_to_category(photo, categories / cmd):
                        break
                else:
                    print("Invalid command")
            except KeyboardInterrupt:
                print("\nStopping categorization")
                return

    print("\nCategorization complete!")

def main():
    parser = argparse.ArgumentParser(description="Manage photo categories")
    parser.add_argument("action", choices=['create', 'list', 'batch'],
                       help="Action to perform")
    parser.add_argument("--base-dir", "-b",
                       default="content/photos/manual-categories",
                       help="Base directory for categories")
    parser.add_argument("--categories", "-c", nargs='+',
                       default=['nature', 'urban', 'people', 'animals', 'food',
                               'travel', 'architecture', 'sunset'],
                       help="List of categories to create")
    parser.add_argument("--source", "-s",
                       help="Source directory for batch categorization")

    args = parser.parse_args()

    # Resolve relative paths
    script_dir = Path(__file__).parent
    base_dir = script_dir.parent / args.base_dir

    if args.action == 'create':
        print(f"Creating category structure in {base_dir}")
        create_category_structure(str(base_dir), args.categories)
        print("\nCategory folders created!")
        print("You can now add photos to these folders manually")

    elif args.action == 'list':
        list_categories(str(base_dir))

    elif args.action == 'batch':
        if not args.source:
            source_dir = script_dir.parent / "content/photos"
        else:
            source_dir = script_dir.parent / args.source

        print(f"Starting batch categorization from {source_dir}")
        batch_categorize(str(source_dir), str(base_dir))

if __name__ == "__main__":
    main()