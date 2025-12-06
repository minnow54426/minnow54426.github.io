# CLAUDE.md

Personal blog with Pelican static site generator, focused on photography categorized galleries.

## Key Commands

### Photo Management
```bash
# Add photos to category
cp photos/*.jpg content/content/photos/manual-categories/nature/

# Generate galleries only
python script/generate_galleries.py --categories-dir content/content/photos/manual-categories --output-dir content/content/photography

# Full deployment (galleries + build + deploy)
python script/deploy.py
```

### Development
```bash
# From project root
cd content && pelican content -s pelicanconf.py -o ../output
python -m http.server 8000 --directory output
```

### Production Build
```bash
cd content && pelican content -s publishconf.py -o ../output
```

## Directory Structure

```
/
├── content/                    # Pelican project
│   ├── content/               # Blog content
│   │   └── photography/       # Photo posts
│   ├── photos/                # Original photos (flat)
│   ├── content/photos/manual-categories/  # Organized by category
│   ├── plugins/photo_gallery.py
│   ├── theme/                 # Custom theme
│   ├── pelicanconf.py         # Dev config
│   └── publishconf.py         # Prod config
├── script/                    # Automation scripts
│   ├── generate_galleries.py  # Create photo galleries
│   └── deploy.py             # Full deployment
└── output/                    # Generated site
```

## Photo Categories

Active categories in `content/content/photos/manual-categories/`:
- animals, architecture, food, nature, people, sunset, travel, urban

## Deployment Process

1. Photos are organized manually in category folders
2. `generate_galleries.py` creates thumbnails and markdown pages
3. Pelican builds static HTML
4. `deploy.py` pushes to GitHub Pages

## Tech Stack

- **Pelican** - Static site generator
- **Python 3** - Scripts and automation
- **PIL/Pillow** - Image thumbnails
- **ghp-import** - GitHub Pages deployment
- **GitHub Pages** - Hosting


# User guide
Everytime you change something, start the server locally and tell me to review it, if it's ok, i will say do it, which means push to remote