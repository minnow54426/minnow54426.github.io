#!/usr/bin/env python3
"""
Script to organize photos by year and month based on file modification dates
"""

import os
import shutil
from pathlib import Path
from datetime import datetime
import re

def get_photo_date(file_path):
    """
    Extract date from filename or file modification time
    
    Args:
        file_path (Path): Path to the photo file
        
    Returns:
        datetime: Date of the photo
    """
    filename = file_path.name
    
    # Try to extract date from filename (DSC or IMG patterns)
    # Look for patterns like DSC01234 or IMG_1234
    date_match = re.search(r'(DSC|IMG)[_]?(\d+)', filename)
    if date_match:
        # For these files, we'll use the file modification date
        pass
    
    # Try to extract date from --export-by-date pattern
    date_match = re.search(r'--export-by-date', filename)
    if date_match:
        # Use file modification date
        mtime = os.path.getmtime(file_path)
        return datetime.fromtimestamp(mtime)
    
    # Default to file modification date
    mtime = os.path.getmtime(file_path)
    return datetime.fromtimestamp(mtime)

def organize_photos(source_dir, target_dir):
    """
    Organize photos by year and month
    
    Args:
        source_dir (str): Directory containing photos to organize
        target_dir (str): Directory to organize photos into
    """
    source_path = Path(source_dir)
    target_path = Path(target_dir)
    
    # Create target directory if it doesn't exist
    target_path.mkdir(parents=True, exist_ok=True)
    
    # Process all image files
    image_extensions = {'.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', '.heic', '.dng', '.arw', '.mov'}
    
    for file_path in source_path.iterdir():
        if file_path.is_file() and file_path.suffix.lower() in image_extensions:
            # Get photo date
            photo_date = get_photo_date(file_path)
            
            # Create year/month directory structure
            year_dir = target_path / str(photo_date.year)
            month_dir = year_dir / f"{photo_date.month:02d}"
            
            # Copy file to organized directory
            target_file = month_dir / file_path.name
            # Create directory and copy file in one step
            target_file.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(file_path, target_file)
            print(f"Copied {file_path.name} to {month_dir}")

def main():
    source_dir = "/Users/boycrypt/code/python/website/content/content/photos"
    target_dir = "/Users/boycrypt/code/python/website/content/content/photos/organized"
    
    print("Organizing photos by year and month...")
    organize_photos(source_dir, target_dir)
    print("Photo organization complete!")

if __name__ == "__main__":
    main()