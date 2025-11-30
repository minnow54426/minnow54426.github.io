"""
Photo Gallery Plugin for Pelican
"""
import os
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
        photos.sort()  # Sort photos alphabetically
        
        # Add photos to all articles context
        for article in generator.articles:
            # Check if the article is in the photography category
            if hasattr(article, 'category') and article.category.name.lower() == 'photography':
                article.photos = photos

def register():
    signals.article_generator_finalized.connect(add_photo_gallery)