#!/usr/bin/env python3
"""
Script to export all original photos from Apple Photos library
"""

import subprocess
import os
import sys
import argparse
from pathlib import Path
import re


def check_osxphotos_installed():
    """Check if osxphotos is installed"""
    try:
        subprocess.run(["osxphotos", "--version"], 
                      capture_output=True, check=True)
        return True
    except (subprocess.CalledProcessError, FileNotFoundError):
        return False


def install_osxphotos():
    """Install osxphotos using pip"""
    try:
        subprocess.run([sys.executable, "-m", "pip", "install", "osxphotos"], 
                      check=True)
        print("Successfully installed osxphotos")
        return True
    except subprocess.CalledProcessError:
        print("Failed to install osxphotos")
        return False


def get_base_name(filename):
    """
    Extract the base name of a photo file by removing the suffixes
    e.g., "photo--export-by-date (1).jpg" -> "photo"
    """
    # Remove the extension first
    name_without_ext = '.'.join(filename.split('.')[:-1])
    
    # Remove suffixes like --export-by-date and (1), (2), etc.
    base_name = name_without_ext.replace('--export-by-date', '')
    # Remove any trailing spaces and parentheses numbers
    base_name = re.sub(r'\s*\(\d+\)
    """
    Export all original photos from Apple Photos library
    
    Args:
        output_dir (str): Directory to export photos to
        verbose (bool): Whether to show verbose output
    """
    # Create output directory if it doesn't exist
    Path(output_dir).mkdir(parents=True, exist_ok=True)
    
    # Build command - export all photos (removed --shared flag)
    cmd = [
        "osxphotos", "export",
        "--original-suffix", "--export-by-date", "--update"
    ]
    
    if verbose:
        cmd.append("--verbose")
        
    cmd.append(output_dir)
    
    try:
        # Run the export command
        result = subprocess.run(cmd, capture_output=True, text=True)
        
        if result.returncode == 0:
            print(f"Successfully exported photos to {output_dir}")
            if verbose:
                print(result.stdout)
            return True
        else:
            print(f"Error exporting photos: {result.stderr}")
            return False
    except Exception as e:
        print(f"Exception occurred during export: {e}")
        return False


def main():
    parser = argparse.ArgumentParser(description="Export all original photos from Apple Photos library")
    parser.add_argument("output_dir", help="Directory to export photos to")
    parser.add_argument("--verbose", "-v", action="store_true", 
                       help="Show verbose output")
    parser.add_argument("--keep-larger", action="store_true",
                       help="Keep only the larger version of duplicate photos")
    
    args = parser.parse_args()
    
    # Check if osxphotos is installed
    if not check_osxphotos_installed():
        print("osxphotos not found. Installing...")
        if not install_osxphotos():
            print("Failed to install osxphotos. Please install manually with: pip install osxphotos")
            return 1
    
    # Export the photos
    if export_shared_photos(args.output_dir, args.verbose):
        # Remove duplicate photos if requested
        if args.keep_larger:
            remove_duplicate_photos(args.output_dir)
        
        print("Export completed successfully!")
        return 0
    else:
        print("Export failed!")
        return 1


if __name__ == "__main__":
    sys.exit(main()), '', base_name)
    
    return base_name


def remove_duplicate_photos(output_dir):
    """
    Remove duplicate photos, keeping only the one with the larger file size
    
    Args:
        output_dir (str): Directory containing exported photos
    """
    output_path = Path(output_dir)
    if not output_path.exists():
        print(f"Output directory {output_dir} does not exist")
        return
    
    # Group files by their base name
    photo_groups = {}
    for file_path in output_path.iterdir():
        if file_path.is_file() and file_path.suffix.lower() in ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', '.heic']:
            base_name = get_base_name(file_path.name)
            if base_name not in photo_groups:
                photo_groups[base_name] = []
            photo_groups[base_name].append(file_path)
    
    # For each group with multiple files, keep only the largest
    removed_count = 0
    for base_name, files in photo_groups.items():
        if len(files) > 1:
            # Sort files by size (largest first)
            files.sort(key=lambda f: f.stat().st_size, reverse=True)
            
            # Keep the largest file and remove the rest
            for file_path in files[1:]:
                print(f"Removing duplicate {file_path.name} (smaller version)")
                file_path.unlink()
                removed_count += 1
    
    print(f"Removed {removed_count} duplicate photos, keeping only the higher quality versions")


def export_shared_photos(output_dir, verbose=False):
    """
    Export all original photos from Apple Photos library
    
    Args:
        output_dir (str): Directory to export photos to
        verbose (bool): Whether to show verbose output
    """
    # Create output directory if it doesn't exist
    Path(output_dir).mkdir(parents=True, exist_ok=True)
    
    # Build command - export all photos (removed --shared flag)
    cmd = [
        "osxphotos", "export",
        "--original-suffix", "--export-by-date", "--update"
    ]
    
    if verbose:
        cmd.append("--verbose")
        
    cmd.append(output_dir)
    
    try:
        # Run the export command
        result = subprocess.run(cmd, capture_output=True, text=True)
        
        if result.returncode == 0:
            print(f"Successfully exported photos to {output_dir}")
            if verbose:
                print(result.stdout)
            return True
        else:
            print(f"Error exporting photos: {result.stderr}")
            return False
    except Exception as e:
        print(f"Exception occurred during export: {e}")
        return False


def main():
    parser = argparse.ArgumentParser(description="Export all original photos from Apple Photos library")
    parser.add_argument("output_dir", help="Directory to export photos to")
    parser.add_argument("--verbose", "-v", action="store_true", 
                       help="Show verbose output")
    parser.add_argument("--keep-larger", action="store_true",
                       help="Keep only the larger version of duplicate photos")
    
    args = parser.parse_args()
    
    # Check if osxphotos is installed
    if not check_osxphotos_installed():
        print("osxphotos not found. Installing...")
        if not install_osxphotos():
            print("Failed to install osxphotos. Please install manually with: pip install osxphotos")
            return 1
    
    # Export the photos
    if export_shared_photos(args.output_dir, args.verbose):
        # Remove duplicate photos if requested
        if args.keep_larger:
            remove_duplicate_photos(args.output_dir)
        
        print("Export completed successfully!")
        return 0
    else:
        print("Export failed!")
        return 1


if __name__ == "__main__":
    sys.exit(main())