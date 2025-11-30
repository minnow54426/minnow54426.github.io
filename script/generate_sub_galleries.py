#!/usr/bin/env python3
"""
Script to generate sub-gallery pages for each year/month
"""

import os
from pathlib import Path
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

def generate_gallery_page(year, month, photo_files, output_dir):
    """
    Generate a gallery page for a specific year/month
    
    Args:
        year (str): Year (e.g., "2025")
        month (str): Month (e.g., "10")
        photo_files (list): List of photo filenames
        output_dir (str): Directory to save the gallery page
    """
    # Month names for better readability
    month_names = {
        "01": "January", "02": "February", "03": "March", "04": "April",
        "05": "May", "06": "June", "07": "July", "08": "August",
        "09": "September", "10": "October", "11": "November", "12": "December"
    }
    
    month_name = month_names.get(month, month)
    
    # Create the markdown content
    content = f"""Title: {month_name} {year} Photo Gallery
Date: 2025-11-15
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
        # Skip non-image files for display (like .db files)
        if photo.lower().endswith(('.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', '.heic', '.dng', '.arw')):
            # Create thumbnail directory
            thumbnails_dir = Path(output_dir) / f"{year}" / f"{month}" / "thumbnails"
            thumbnails_dir.mkdir(parents=True, exist_ok=True)
            
            # Create thumbnail
            photo_path = Path(f"/Users/boycrypt/code/python/website/content/content/photos/organized/{year}/{month}/{photo}")
            thumbnail_path = thumbnails_dir / photo
            thumbnail_url = f"/photography/{year}/{month}/thumbnails/{photo}"
            
            # Create thumbnail if it doesn't exist
            if not thumbnail_path.exists():
                original_photo_path = Path(f"/Users/boycrypt/code/python/website/content/content/photos/organized/{year}/{month}/{photo}")
                if original_photo_path.exists():
                    create_thumbnail(original_photo_path, thumbnail_path, (300, 300))
            
            # Add photo to gallery with lightbox functionality
            content += f"""  <div class="photo-item">
    <a href="/photos/organized/{year}/{month}/{photo}" class="lightbox-link" data-lightbox="gallery">
      <img src="{thumbnail_url}" alt="{photo}" />
    </a>
  </div>
"""
    
    # Close the gallery div and add CSS and JavaScript for lightbox
    content += """</div>

<style>
  .photo-gallery {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    grid-gap: 20px;
    margin-top: 20px;
  }
  
  .photo-item {
    overflow: hidden;
    border-radius: 5px;
    box-shadow: 0 2px 5px rgba(0,0,0,0.1);
  }
  
  .photo-item img {
    width: 100%;
    height: auto;
    display: block;
    transition: transform 0.3s ease;
  }
  
  .photo-item img:hover {
    transform: scale(1.05);
  }
  
  /* Lightbox styles */
  .lightbox-overlay {
    display: none;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.8);
    z-index: 1000;
    justify-content: center;
    align-items: center;
  }
  
  .lightbox-content {
    max-width: 90%;
    max-height: 90%;
  }
  
  .lightbox-content img {
    max-width: 100%;
    max-height: 80vh;
    border-radius: 5px;
  }
  
  .lightbox-close {
    position: absolute;
    top: 20px;
    right: 30px;
    color: white;
    font-size: 30px;
    cursor: pointer;
  }
</style>

<script>
  // Simple lightbox implementation
  document.addEventListener('DOMContentLoaded', function() {
    const lightboxLinks = document.querySelectorAll('.lightbox-link');
    const lightboxOverlay = document.createElement('div');
    lightboxOverlay.className = 'lightbox-overlay';
    document.body.appendChild(lightboxOverlay);
    
    const lightboxContent = document.createElement('div');
    lightboxContent.className = 'lightbox-content';
    lightboxOverlay.appendChild(lightboxContent);
    
    const closeBtn = document.createElement('span');
    closeBtn.className = 'lightbox-close';
    closeBtn.innerHTML = '&times;';
    lightboxOverlay.appendChild(closeBtn);
    
    lightboxLinks.forEach(link => {
      link.addEventListener('click', function(e) {
        e.preventDefault();
        const imgSrc = this.href;
        const img = document.createElement('img');
        img.src = imgSrc;
        lightboxContent.innerHTML = '';
        lightboxContent.appendChild(img);
        lightboxOverlay.style.display = 'flex';
        document.body.style.overflow = 'hidden';
      });
    });
    
    closeBtn.addEventListener('click', function() {
      lightboxOverlay.style.display = 'none';
      document.body.style.overflow = 'auto';
    });
    
    lightboxOverlay.addEventListener('click', function(e) {
      if (e.target === lightboxOverlay) {
        lightboxOverlay.style.display = 'none';
        document.body.style.overflow = 'auto';
      }
    });
  });
</script>
"""
    
    # Write the file
    output_path = Path(output_dir) / f"{year}" / f"{month}"
    output_path.mkdir(parents=True, exist_ok=True)
    
    gallery_file = output_path / "index.md"
    with open(gallery_file, "w") as f:
        f.write(content)
    
    print(f"Generated gallery page for {year}/{month} with {len(photo_files)} photos")

def check_photos_exist(year, month, photo_files):
    """
    Check if the photos referenced in the gallery actually exist
    
    Args:
        year (str): Year (e.g., "2025")
        month (str): Month (e.g., "10")
        photo_files (list): List of photo filenames
        
    Returns:
        bool: True if all photos exist, False otherwise
    """
    for photo in photo_files:
        if photo.lower().endswith(('.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', '.heic', '.dng', '.arw')):
            photo_path = Path(f"/Users/boycrypt/code/python/website/content/content/photos/organized/{year}/{month}/{photo}")
            if not photo_path.exists():
                return False
    return True

def generate_all_galleries(organized_photos_dir, output_dir):
    """
    Generate gallery pages for all year/month directories
    
    Args:
        organized_photos_dir (str): Directory with organized photos (year/month structure)
        output_dir (str): Directory to save gallery pages
    """
    organized_path = Path(organized_photos_dir)
    
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
                    
                    # Generate gallery page only if there are photos and they exist
                    if photo_files and check_photos_exist(year, month, photo_files):
                        generate_gallery_page(year, month, photo_files, output_dir)
                    else:
                        # Remove empty month directory from both organized photos and gallery
                        print(f"Removing empty month directory: {year}/{month}")
                        import shutil
                        shutil.rmtree(month_dir)
                        
                        # Also remove the gallery directory if it exists
                        gallery_dir = Path(output_dir) / f"{year}" / f"{month}"
                        if gallery_dir.exists():
                            shutil.rmtree(gallery_dir)

def main():
    organized_photos_dir = "/Users/boycrypt/code/python/website/content/content/photos/organized"
    output_dir = "/Users/boycrypt/code/python/website/content/content/photography"
    
    print("Generating sub-gallery pages...")
    generate_all_galleries(organized_photos_dir, output_dir)
    print("Gallery page generation complete!")

if __name__ == "__main__":
    main()