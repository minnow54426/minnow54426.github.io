# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with this personal blog repository.

## Project Overview

A sophisticated personal blog built with **Pelican** static site generator, featuring a manual photo categorization system with custom gallery pages. The site showcases photography through folder-based organization with lightbox functionality.

**Live URL**: https://minnow54426.github.io
**Author**: cryptboy
**Site Name**: wonderonpathlesspath

## Quick Start

### Photo Management Workflow
From the `content/` directory:
```bash
# 1. Add photos to category folders
cp your_photos/*.jpg content/photos/manual-categories/nature/

# 2. Generate galleries (creates thumbnails and index pages)
python ../script/generate_category_galleries.py

# 3. Generate folder index pages (for direct folder access)
python ../script/generate_folder_indexes.py

# 4. Build and deploy
pelican content -s publishconf.py -o output
cp -r content/photos/manual-categories output/photos/
ghp-import output -b gh-pages -p
```

### One-Click Deployment (All-in-One)
```bash
# From content directory - handles everything
python ../script/deploy_with_photos.py
```

### Development Workflow
From the `content/` directory:
```bash
pelican content -s pelicanconf.py -o output  # Generate site
pelican content -s publishconf.py -o output  # Build for production
python -m http.server 8000 output              # Local server (port 8000)
```

## Core Features

### 📸 Photo Management System
A sophisticated photo organization and gallery system:

**Photo Processing Scripts** (run from project root unless specified):
- `organize_photos.py` - Organizes photos by year/month hierarchy
  - Options: `--source`, `--target`, `--move` (to move instead of copy)
- `generate_sub_galleries.py` - Creates thumbnails and gallery markdown pages
  - Options: `--photos-dir`, `output-dir`
- `generate_category_galleries.py` - Creates category-based galleries
  - Options: `--categories-dir`, `--output-dir`
- `generate_folder_indexes.py` - Creates index.html for folder browsing
  - Options: `--categories-dir`, `output-dir`
- `categorize_photos.py` - Interactive photo categorization helper
  - Actions: `create`, `list`, `batch`
- `deploy_with_photos.py` - Orchestrates full deployment pipeline

**Key Features**:
- Hierarchical organization (year/month structure)
- Automatic thumbnail generation with PIL
- Multi-format support (.jpg, .png, .gif, .webp, .heic, .dng, .arw, .mov)
- Masonry grid gallery with advanced lightbox functionality
- Keyboard navigation (arrow keys, escape)
- Touch/swipe support for mobile
- External CSS/JS for better maintainability

### 🎨 Gallery System
- Masonry grid layout with responsive design
- Lightbox functionality for full-size viewing
- Hover effects and smooth transitions
- Automatic photo detection and injection

## Architecture

### Directory Structure
```
/Users/boycrypt/code/python/website/
├── script/                     # Photo processing automation
│   ├── organize_photos.py
│   ├── generate_sub_galleries.py
│   ├── generate_category_galleries.py
│   ├── generate_folder_indexes.py
│   ├── categorize_photos.py
│   └── deploy_with_photos.py
├── content/                    # Main Pelican project
│   ├── content/               # Blog posts by category
│   │   └── photography/       # Photography posts (active)
│   ├── photos/                # Photo assets
│   │   └── manual-categories/  # Manually organized photos
│   │       ├── animals/
│   │       ├── architecture/
│   │       ├── food/
│   │       ├── nature/
│   │       ├── people/
│   │       ├── sunset/
│   │       ├── travel/
│   │       └── urban/
│   ├── plugins/               # Custom Pelican plugins
│   │   └── photo_gallery.py   # Photo gallery integration
│   ├── theme/                 # Custom theme and templates
│   │   ├── css/               # Stylesheets
│   │   │   └── gallery.css    # Gallery-specific styles
│   │   └── js/                # JavaScript files
│   │       └── gallery.js     # Gallery lightbox functionality
│   ├── pelicanconf.py         # Development config
│   └── publishconf.py         # Production config
└── output/                    # Generated static site
```

### Content Structure
Posts use Markdown with YAML frontmatter:
```yaml
Title: Post Title
Date: YYYY-MM-DD
Category: Photography  # Currently active category
Tags: tag1, tag2
Slug: url-slug
Author: Cryptboy
Summary: Brief description
```

## Technology Stack

### Core Technologies
- **Pelican** - Python static site generator
- **Python 3** - Automation and scripting
- **Markdown** - Content format
- **HTML/CSS/JavaScript** - Frontend with custom gallery

### Key Dependencies
- **ghp-import** - GitHub Pages deployment
- **PIL (Pillow)** - Image processing for thumbnails

### Platform Requirements
- **Python 3.8+** - For scripts and Pelican
- **Cross-platform** - Works on Windows, macOS, and Linux

## Deployment Process

1. **Photo Organization**: Existing photos are organized by year/month
2. **Gallery Generation**: Thumbnails and gallery HTML are created
3. **Site Build**: Pelican generates static HTML with photo galleries
4. **Deploy**: ghp-import pushes to GitHub Pages

**Working Directory**: Always run commands from `content/` except photo scripts (run from root)

## Development Notes

### Current Status
- **Photography**: Fully implemented with automated gallery system
- **Code/Music**: Categories prepared but not yet utilized
- **Theme**: Custom responsive design with photo focus

### Plugin System
The `photo_gallery` plugin automatically:
- Detects photos in `/photos` directory
- Injects galleries into Photography category posts
- Sorts photos alphabetically
- Handles multiple image formats

## Git Configuration

- **Remote**: https://github.com/minnow54426/minnow54426.github.io.git
- **Target**: GitHub Pages user site
- **Deployment**: ghp-import handles gh-pages branch automatically
- **Main Branch**: Contains source code and content