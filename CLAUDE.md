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

### Git Workflow (Complete Guide)
```bash
# Step 1: Check current status and see what changed
git status

# Step 2: Review changes before committing
git diff  # See unstaged changes
git diff --staged  # See staged changes

# Step 3: Add files to staging area
git add .  # Add all changes
# OR specific files: git add index.html photo-gallery.html

# Step 4: Commit with descriptive message
git commit -m "Brief description of changes

Detailed explanation of what was changed and why.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"

# Step 5: Push to ALL branches (GitHub Pages uses main as default)
git push origin main     # Primary branch for GitHub Pages
git push origin master   # Backup/legacy branch
git push origin gh-pages # Additional backup branch

# Step 6: Verify deployment
# Wait 1-2 minutes for GitHub Pages to rebuild
# Check: https://your-username.github.io/
```

### Branch Management
```bash
# Check current branch
git branch

# Switch to main branch (GitHub Pages default)
git checkout main

# Merge changes from other branches if needed
git merge master --no-ff

# Check which branch GitHub Pages uses
git remote show origin  # Look for "HEAD branch"
```

### Troubleshooting Git Issues
```bash
# If push fails due to large files (>100MB)
# 1. Remove large files from git tracking
git rm --cached large-file.mp4
echo "large-file.mp4" >> .gitignore
git add .gitignore
git commit -m "Remove large file and add to gitignore"

# 2. Or use Git LFS for large files (advanced)
git lfs track "*.mp4"
git add .gitattributes
git add large-file.mp4
git commit -m "Add large file with LFS"

# If network issues occur
git config http.postBuffer 524288000  # Increase buffer
git remote remove origin
git remote add origin https://github.com/username/repo.git

# If branches are out of sync
git fetch origin
git reset --hard origin/main  # Use with caution!
```

### Photo Management
```bash
# Add new photos to the gallery
cp /path/to/new/photos/*.jpg photos/

# Update photo-gallery.html to include new photos
# (Edit the file manually to add new photo items)

# Test locally, then commit and push
```

### Video & Media Management
```bash
# Video Compression (for web optimization)
ffmpeg -i original.mov -c:v libx264 -preset medium -crf 28 -movflags +faststart compressed.mp4

# Ultra-compress for very fast loading (935KB for 2:20 video)
ffmpeg -i original.mp4 -c:v libx264 -preset veryfast -crf 35 -vf "scale=720:1280" -movflags +faststart ultra_compressed.mp4

# Standard compression (6.1MB for 2:20 video) - RECOMMENDED
ffmpeg -i original.mov -c:v libx264 -preset medium -crf 28 -movflags +faststart standard_compressed.mp4

# Check video properties
ffprobe -v quiet -print_format json -show_format -show_streams video.mp4

# Check file sizes
ls -lh videos/
```

### Content Guidelines
```bash
# File size recommendations for GitHub Pages:
# - Images: Under 5MB each
# - Videos: Under 10MB each (GitHub limit is 100MB)
# - Total repository: Under 1GB recommended

# File format recommendations:
# - Images: JPG, PNG, WebP
# - Videos: MP4 (H.264), WebM
# - Documents: PDF, MD

# Organization:
# - photos/ for photography
# - paint/ for artwork and videos
# - Keep directory names lowercase with underscores
```

## Directory Structure

```
/
├── index.html              # Main page with navigation sections
├── photo-gallery.html      # Photo gallery with folder navigation
├── paint.html              # Paint gallery with video support
├── code.html               # Code projects page
├── music.html              # Music projects page
├── photos/                 # Photography collection
│   ├── animals/           # Animal photos
│   ├── shang\ hai/        # Shanghai photos
│   ├── jiu\ zhai\ gou/    # Jiu Zhai Gou photos
│   └── [other folders]/   # Additional photo categories
├── paint/                  # Art and video collection
│   ├── water_color/       # Watercolor paintings and videos
│   └── README.md          # Video upload instructions
├── _config.yml            # Jekyll/GitHub Pages configuration
├── .nojekyll              # Disable Jekyll processing
├── .gitignore             # Files to exclude from git
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
- Folder-based navigation with expandable sections
- Masonry-style layout with varied photo sizes
- Lightbox functionality for full-size viewing
- "← Back to Home" link for navigation

### Paint Gallery (`paint.html`)
- Folder-based navigation similar to photo gallery
- Video support with optimized playback
- Expandable watercolor and other art categories
- Responsive video grid layout
- Advanced seeking support for .mov files
- "← Back to Home" link for navigation

### Photo Layout
- Grid system with `repeat(auto-fill, minmax(250px, 1fr))`
- Compact `grid-gap: 2px` for tight spacing
- Responsive design for mobile and tablet
- Hover effects and smooth transitions

## Deployment Process (Learned from Experience)

### Step-by-Step Workflow
1. **Local Development**: Make changes to HTML/CSS/JS files
2. **Local Testing**: Start server and thoroughly test changes
3. **Git Staging**: Review and stage changes carefully
4. **Git Commit**: Commit with descriptive, detailed message
5. **Multi-Branch Push**: Push to main, master, and gh-pages
6. **Verification**: Wait 1-2 minutes and check live site

### Critical Lessons Learned
- **ALWAYS push to ALL branches**: GitHub Pages may use main, master, or gh-pages
- **Check Jekyll config**: Update `_config.yml` include list for new files
- **File size limits**: GitHub has 100MB file limit, aim for under 10MB
- **Network issues**: May need multiple push attempts or manual upload
- **Branch synchronization**: Keep all branches consistent

### Common Pitfalls & Solutions
1. **Files not appearing online** → Check `_config.yml` include list
2. **Large file push fails** → Compress files or use manual upload
3. **Wrong branch deployed** → Verify GitHub Pages default branch
4. **Changes not visible** → Clear browser cache, wait longer for deployment

### Quick Deployment Checklist
```bash
# Before every deployment:
□ Local server running: python -m http.server 8001
□ All changes tested locally
□ git status shows expected changes
□ File sizes under 10MB for media
□ _config.yml includes new directories
□ Committed with proper message format
□ Pushed to all branches
□ Wait 2 minutes before checking live site
```

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

## User Guide (Updated Workflow)

### Standard Update Process
Every time you change something:
1. Start the local server: `python -m http.server 8001`
2. Test changes at http://localhost:8001/
3. Review the changes thoroughly (all pages, links, functionality)
4. If satisfied, say "do it" to push to remote
5. I will commit and push the changes to ALL GitHub Pages branches

### For Large Media Files
1. Compress videos/images before adding
2. Test locally with compressed versions
3. If push fails due to size, use manual upload via GitHub web interface
4. Update HTML to reference uploaded file

### Adding New Content Sections
1. Create new HTML page (e.g., new-section.html)
2. Update index.html to add navigation
3. Update _config.yml to include new page
4. Test all navigation links
5. Commit and push to all branches

### Emergency Recovery
If deployment fails:
1. Check which branch GitHub Pages uses: `git remote show origin`
2. Ensure files are in _config.yml include list
3. Verify file sizes are under GitHub limits
4. Try manual upload for large files
5. Clear browser cache and wait 2-5 minutes

## Important Notes

- **No Pelican**: This is now a static HTML site, not Pelican-based
- **Direct Deployment**: Files are served directly from the root directory
- **No Build Process**: Changes are immediately reflected after push
- **Clean Structure**: No duplicate directories or build artifacts
- **GitHub Pages Optimized**: Proper configuration for immediate deployment
- **Legacy Content**: The `/content/` folder was removed as it was from old Pelican setup; the site uses direct `/photos/` and `/paint/` folders