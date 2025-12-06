# CLAUDE.md

Personal photography website with GitHub Pages, featuring a clean photo gallery with compact layout.

## Key Commands

### Development & Testing
```bash
# Start local development server
python -m http.server 8001

# View local site
# Main page: http://localhost:8001/
# Photo gallery: http://localhost:8001/photo-gallery.html
```

### Git Workflow
```bash
# Check current status
git status

# Add and commit changes
git add .
git commit -m "Descriptive commit message"

# Push to GitHub Pages (both branches)
git push origin master
git push origin master:main
```

### Photo Management
```bash
# Add new photos to the gallery
cp /path/to/new/photos/*.jpg photos/

# Update photo-gallery.html to include new photos
# (Edit the file manually to add new photo items)

# Test locally, then commit and push
```

## Directory Structure

```
/
├── index.html              # Main page with elegant design
├── photo-gallery.html      # Photo gallery with compact 2px spacing
├── photos/                 # All photos (flat structure)
│   ├── *.jpg              # Photo files
│   └── *.JPG              # Photo files
├── _config.yml            # Jekyll/GitHub Pages configuration
├── .nojekyll              # Disable Jekyll processing
├── README.md              # Project documentation
└── CLAUDE.md              # This file - AI assistant instructions
```

## Website Structure

### Main Page (`index.html`)
- Clean, elegant design with centered layout
- Three main sections: Photography, Code, Music
- Navigation buttons with hover effects
- Links directly to `/photo-gallery.html`

### Photo Gallery (`photo-gallery.html`)
- Minimal design with no header text
- Compact 2px grid spacing between photos
- Masonry-style layout with varied photo sizes
- Lightbox functionality for full-size viewing
- "← Back to Home" link for navigation

### Photo Layout
- Grid system with `repeat(auto-fill, minmax(250px, 1fr))`
- Compact `grid-gap: 2px` for tight spacing
- Responsive design for mobile and tablet
- Hover effects and smooth transitions

## Deployment Process

1. **Local Development**: Make changes to HTML files
2. **Local Testing**: Start server and review changes
3. **Git Commit**: Commit changes with descriptive message
4. **Push to Remote**: Push to both master and main branches
5. **GitHub Pages**: Automatic deployment within 1-2 minutes

## Tech Stack

- **HTML5/CSS3** - Modern web standards
- **CSS Grid** - Photo gallery layout
- **JavaScript** - Lightbox functionality
- **GitHub Pages** - Static hosting
- **Git** - Version control and deployment

## Photo Gallery Features

- **Compact Layout**: 2px spacing between photos
- **Responsive Design**: Adapts to all screen sizes
- **Lightbox**: Click photos for full-screen viewing
- **Hover Effects**: Subtle animations on interaction
- **Masonry Grid**: Dynamic, Pinterest-style layout

## User Guide

Every time you change something:
1. Start the local server: `python -m http.server 8001`
2. Test changes at http://localhost:8001/
3. Review the changes thoroughly
4. If satisfied, say "do it" to push to remote
5. I will commit and push the changes to GitHub Pages

## Important Notes

- **No Pelican**: This is now a static HTML site, not Pelican-based
- **Direct Deployment**: Files are served directly from the root directory
- **No Build Process**: Changes are immediately reflected after push
- **Clean Structure**: No duplicate directories or build artifacts
- **GitHub Pages Optimized**: Proper configuration for immediate deployment