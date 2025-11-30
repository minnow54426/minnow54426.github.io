"""
Photo Gallery Plugin for Pelican
"""
import os
import random
from pelican import signals
from pelican.contents import Article

def add_photo_gallery(generator):
    """Add photo gallery to photo articles"""
    # Get list of photos from the photos directory
    photos_dir = os.path.join(generator.path, 'photos')
    if os.path.exists(photos_dir):
        photos = []
        for file in os.listdir(photos_dir):
            if file.lower().endswith(('.png', '.jpg', '.jpeg', '.gif', '.bmp', '.tiff', '.webp', '.heic')):
                photos.append(file)

        # Sort photos but maintain some randomness for visual variety
        photos.sort()
        # Add some controlled randomness for better visual arrangement
        random.shuffle(photos[:min(20, len(photos))])  # Shuffle first 20 photos

        # Display all available photos - no limit
        # max_photos = 50  # Removed limit to show all photos

        # Add photos to all articles context
        for article in generator.articles:
            # Check if the article is in the photography category
            if hasattr(article, 'category') and article.category.name.lower() == 'photography':
                article.photos = photos
                article.total_photos = len(photos)  # Add total count for template use

def register():
    signals.article_generator_finalized.connect(add_photo_gallery)