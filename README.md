# Personal Blog - Photography Focus

A clean Pelican-based blog showcasing photography through categorized galleries.

**Live Site**: https://minnow54426.github.io

## Quick Start

### Add New Photos
```bash
# 1. Copy photos to category folder
cp your_photos/*.jpg content/content/photos/manual-categories/nature/

# 2. Generate galleries and deploy
python script/deploy.py
```

### Local Development
```bash
# Generate site locally
cd content && pelican content -s pelicanconf.py -o ../output

# Serve locally
python -m http.server 8000 --directory output
```

## Photo Management

### Directory Structure
```
content/content/photos/manual-categories/
├── animals/      # Animal photography
├── architecture/ # Buildings & structures
├── food/         # Food photography
├── nature/       # Nature & landscapes
├── people/       # People & portraits
├── sunset/       # Sunset photos
├── travel/       # Travel photography
└── urban/        # Urban scenes
```

### Scripts
- `script/generate_galleries.py` - Creates thumbnails and gallery pages
- `script/deploy.py` - Full deployment pipeline

### Adding New Categories
1. Create directory: `mkdir content/content/photos/manual-categories/newcategory`
2. Add photos to the directory
3. Run: `python script/deploy.py`

## Configuration

### Pelican Config Files
- `content/pelicanconf.py` - Development settings
- `content/publishconf.py` - Production settings

### Theme
- Custom responsive theme in `content/theme/`
- Gallery CSS: `content/theme/css/gallery.css`
- Gallery JS: `content/theme/js/gallery.js`

## Deployment

The site deploys automatically to GitHub Pages when you run:
```bash
python script/deploy.py
```

This script:
1. Generates photo galleries
2. Builds the site with Pelican
3. Copies photo directories
4. Pushes to GitHub Pages

## Dependencies

- Python 3.8+
- Pelican: `pip install pelican`
- ghp-import: `pip install ghp-import`
- Pillow: `pip install Pillow`

## Content Structure

Blog posts go in `content/content/{category}/` with YAML frontmatter:

```yaml
Title: Post Title
Date: 2025-12-06
Category: Photography
Tags: tag1, tag2
Slug: url-slug
Author: Cryptboy
Summary: Brief description
```