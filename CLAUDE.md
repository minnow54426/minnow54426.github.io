# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Personal portfolio website with GitHub Pages, featuring photography, code projects (including Zero Knowledge Proof learning), and interactive cryptography visualizations.

## Key Commands

### Development & Testing
```bash
# Start local development server (from repository root)
python -m http.server 8001

# View local site
# Main page: http://localhost:8001/
# Photo gallery: http://localhost:8001/photo-gallery.html
# Paint gallery: http://localhost:8001/paint.html
# Code projects: http://localhost:8001/code.html
# Cryptography: http://localhost:8001/cryptography.html
# Music: http://localhost:8001/music.html
```

### Rust Projects (Zero Knowledge Proof Learning)
```bash
# Navigate to specific week's project
cd code/zero_knowledge_proof_learn/week1/code/rust-protocol-basics

# Build the project
cargo build

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run clippy for linting
cargo clippy

# Format code
cargo fmt

# Run binary (hash-cli example)
cargo run --bin hash-cli "hello world"

# Run examples
cargo run --example basic_usage

# Generate documentation
cargo doc --open

# Run benchmarks
cargo bench
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
# Add new photos to existing category
cp /path/to/new/photos/*.jpg "photos/shang hai/"

# Regenerate gallery HTML (if needed)
cd photos
python3 << 'EOF'
# Simple script to add photos to existing gallery
import os
photo_dir = "shang hai"
photos = sorted([f for f in os.listdir(photo_dir) if f.lower().endswith('.jpg')])
for photo in photos:
    print(f'<a href="{photo_dir}/{photo}" class="image"><img src="{photo_dir}/{photo}" alt="" loading="lazy" /></a>')
EOF

# For new categories, create directory and regenerate all galleries
mkdir "photos/new category"
cp /path/to/photos/*.jpg "photos/new category/"
# Update photo-gallery.html to add new category link
# Regenerate gallery HTML using the template structure

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
├── code.html               # Code projects page with file tree navigation
├── music.html              # Music projects page
├── cryptography.html       # Cryptography learning page
├── polynomial-plotter.html # Standalone polynomial plotter
├── app.js                  # Main JavaScript for site interactions
├── styles.css              # Global styles
├── photos/                 # Photography collection (12 categories)
│   ├── shang hai/         # Shanghai photos (62 JPGs)
│   ├── animals/           # Animal photos (23 JPGs)
│   ├── jiu zhai gou/      # Jiu Zhai Gou photos (20 JPGs)
│   ├── zhang jia jie/     # Zhang Jia Jie photos (13 JPGs)
│   ├── hu pao gong yuan/  # Hu Pao Park photos (14 JPGs)
│   ├── qing dao/          # Qing Dao photos (16 JPGs)
│   ├── nan xun gu zhen/   # Nanxun Ancient Town photos (10 JPGs)
│   ├── qian dao hu/       # Qian Dao Hu photos (10 JPGs)
│   ├── ao men/            # Ao Men photos (7 JPGs)
│   ├── zhu hai/           # Zhu Hai photos (3 JPGs)
│   ├── on road/           # Travel photos (4 JPGs)
│   ├── others/            # Other photos (2 JPGs)
│   ├── shang-hai.html     # Shanghai gallery page
│   ├── animals.html       # Animals gallery page
│   ├── jiu-zhai-gou.html  # Jiu Zhai Gou gallery page
│   ├── zhang-jia-jie.html # Zhang Jia Jie gallery page
│   ├── hu-pao-gong-yuan.html # Hu Pao Park gallery page
│   ├── qing-dao.html      # Qing Dao gallery page
│   ├── nan-xun-gu-zhen.html # Nanxun gallery page
│   ├── qian-dao-hu.html   # Qian Dao Hu gallery page
│   ├── ao-men.html        # Ao Men gallery page
│   ├── zhu-hai.html       # Zhu Hai gallery page
│   ├── on-road.html       # On Road gallery page
│   └── others.html        # Others gallery page
├── paint/                  # Art and video collection
│   └── water_color/       # Watercolor videos (12 videos)
├── code/                   # Code projects directory
│   ├── christmas_tree/    # Christmas tree project
│   ├── interactiva_panel/ # Interactive panel with polynomial plotter
│   └── zero_knowledge_proof_learn/  # ZK learning journey (12-week plan)
│       ├── ZK_learning_plan.md      # Master learning plan
│       ├── week1/         # Rust foundations
│       │   └── code/rust-protocol-basics/  # Hashing, serialization
│       ├── week2/         # Merkle trees
│       │   └── code/merkle-rs/
│       └── week3/         # (TBD)
├── cryptography/           # Cryptography interactive tools
│   ├── app.js             # Polynomial plotter logic
│   ├── polynomial-plotter.html
│   └── styles.css
├── _config.yml            # GitHub Pages configuration
├── .nojekyll              # Disable Jekyll processing
├── .gitignore             # Files to exclude from git
├── README.md              # Project documentation
├── CLAUDE.md              # This file - AI assistant instructions
└── html5up-multiverse.zip # HTML5 UP Multiverse template source
```

## Website Structure

### Main Page (`index.html`)
- Clean, elegant design with centered layout
- Four main sections: Photography, Code, Music, Cryptography
- Navigation buttons with hover effects
- Links to specialized sub-pages

### Code Projects Page (`code.html`)
- File tree navigation for browsing code projects
- Expandable/collapsible folder structure
- Organized by project type:
  - `christmas_tree/` - Christmas tree visualization
  - `interactiva_panel/` - Interactive visualization panel
  - `zero_knowledge_proof_learn/` - 12-week ZK learning journey

### Cryptography Page (`cryptography.html`)
- Interactive cryptographic tools and visualizations
- Links to ZK-SNARK polynomial plotter
- Educational content about Zero Knowledge Proofs
- Applications and real-world use cases

### Zero Knowledge Proof Learning Journey
Located in `code/zero_knowledge_proof_learn/`, this is a structured 12-week learning plan covering:

**Week 1: Rust Protocol Basics** (`week1/code/rust-protocol-basics/`)
- Core cryptographic operations: hashing, serialization, hex encoding
- Type-safe wrappers for cryptographic data
- Comprehensive test suite (23 unit tests)
- CLI tool for hashing operations
- Key modules:
  - `bytes.rs` - Hex encoding/decoding, binary serialization
  - `hash.rs` - SHA-256 hashing
  - `types.rs` - Type-safe wrappers (Hash32)
  - `lib.rs` - Library exports and organization
- Dependencies: hex, bincode, serde, sha2, anyhow, clap
- All tests pass with zero clippy warnings

**Week 2: Merkle Trees** (`week2/code/merkle-rs/`)
- Merkle tree implementation in Rust
- Security analysis documentation

**Week 3+**: Future weeks planned (see `ZK_learning_plan.md` for full curriculum)

**Key Learning Resources:**
- `ZK_learning_plan.md` - Master 12-week curriculum plan
- Individual week README files with detailed instructions
- Focus on blockchain + ZK proofs in Rust

### Photo Gallery (`photo-gallery.html`)
- Main gallery index with 12 photo categories
- Each category links to individual gallery page
- Category cards with simple photo count text (e.g., "62 photos")
- Clean design without progress bars
- Responsive grid layout for category selection
- Links to individual category pages (e.g., `photos/shang-hai.html`)

### Individual Photo Galleries (`photos/*.html`)
- **HTML5 UP Multiverse template** with jQuery Poptrox modal popup
- **Modal popup functionality**: Click any photo to open modal lightbox
  - Full image display in centered modal
  - Close button (×) in top-right corner
  - Click outside image or press ESC to close
  - Next/Prev navigation arrows between photos
- **Flexbox grid** with responsive columns:
  - Desktop: 4 columns (25% width)
  - Medium: 3 columns (33.33% width)
  - Tablet: 2 columns (50% width)
  - Mobile: 1 column (100% width)
- Viewport-based photo height: `calc(40vh - 2em)`
- Fixed header with navigation links
- Gradient overlay effect on hover
- **Shared resources**: `photos/gallery.css` and `photos/gallery.js`
- **Background-image technique**: Images set as CSS backgrounds for performance
- 12 categories: Shanghai (62), Jiu Zhai Gou (20), Zhang Jia Jie (13), Hu Pao Park (14), Qing Dao (16), Nanxun (10), Qian Dao Hu (10), Animals (23), Ao Men (7), Zhu Hai (3), On Road (4), Others (2)

### Paint Gallery (`paint.html`)
- Folder-based navigation with 12 watercolor videos
- Video support with optimized compression (all under 10MB)
- Clean filenames without compression suffixes
- Responsive video grid layout with hover effects
- Advanced seeking support for smooth playback
- "← Back to Home" link for navigation
- Total of 12 videos: Christmas Snowman, Single Leaf, Mountain, Leaf on Water, Flower, Autumn Leave, Rose, Peach, Cherry Blossoms, Swan, Flower Bed, Whale

### Photo Gallery Layout (Multiverse Style)
- **Flexbox grid** with `flex-wrap: wrap`
- **Responsive breakpoints** for automatic column adjustment
- **Viewport-based heights** for consistent photo sizing
- **Article-based structure** (`<article class="thumb">`) for SEO
- **jQuery Poptrox** modal popup for image viewing (via CDN)
- **CSS gradient overlays** on photo hover
- **Fixed header** with backdrop blur effect
- **Background-image technique**: Images loaded as CSS backgrounds for better performance
- **Shared CSS/JS**: `photos/gallery.css` and `photos/gallery.js` for all galleries

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

### Web Frontend
- **HTML5/CSS3** - Modern web standards
- **HTML5 UP Templates** - Multiverse (photo galleries), Ethereal (main site)
- **Flexbox** - Responsive photo gallery layouts
- **jQuery** - DOM manipulation and interactions
- **jQuery Poptrox** - Modal popup functionality for photo galleries (HTML5 UP Multiverse template)
- **Font Awesome** - Icon system
- **JavaScript** - Interactive galleries and visualizations
- **GitHub Pages** - Static hosting
- **Git** - Version control and deployment

### Rust Projects
- **Rust 2021 Edition** - Modern Rust with latest features
- **Cargo** - Package manager and build system
- **Key Crates**:
  - `hex` - Hex encoding/decoding
  - `bincode` - Binary serialization
  - `serde` - Serialization framework
  - `sha2` - SHA-256 hashing
  - `anyhow` - Error handling
  - `clap` - CLI argument parsing
  - `criterion` - Benchmarking

## Code Architecture

### Web Site Structure
The site is organized as a static HTML site with no build process:
- Each major section has its own HTML page
- JavaScript in `app.js` handles interactive elements
- `styles.css` provides global styling
- Media files (photos, videos) in organized directories
- Code projects in `/code/` with subdirectories for each project

### Rust Project Architecture (ZK Learning)

**Module Organization** (using Week 1 as example):
```
rust-protocol-basics/
├── src/
│   ├── lib.rs           # Public API exports
│   ├── bytes.rs         # Hex encoding/decoding
│   ├── hash.rs          # SHA-256 hashing
│   ├── types.rs         # Type-safe wrappers
│   └── bin/
│       └── hash-cli.rs  # CLI tool
├── examples/
│   └── basic_usage.rs   # Usage examples
├── benches/             # Criterion benchmarks
├── tests/               # Integration tests
└── Cargo.toml           # Project config
```

**Key Architectural Patterns**:
1. **Library + Binary Pattern**: Each crate has library code (`src/`) and optional binaries (`src/bin/`)
2. **Type Safety**: Newtype pattern for cryptographic types (e.g., `Hash32`)
3. **Error Handling**: `anyhow::Result<T>` for application errors
4. **Testing**: Comprehensive unit tests in each module, doctests for examples
5. **Documentation**: Rust doc comments with examples
6. **Serialization**: `serde` + `bincode` for deterministic binary encoding

**Working with ZK Learning Projects**:
- Each week is a standalone Rust project
- Navigate to the specific week's directory before running cargo commands
- Each project has its own `Cargo.toml` with specific dependencies
- README files in each project explain the learning objectives
- Tests are the primary way to verify understanding
- Clippy is used to ensure code quality

## Development Workflow

### Adding New Web Content
1. Create HTML file in root directory
2. Add navigation links in `index.html`
3. Update `_config.yml` to include new file for GitHub Pages
4. Test locally with `python -m http.server 8001`
5. Commit and push to all branches (main, master, gh-pages)

### Adding New Rust Code (ZK Learning)
1. Navigate to appropriate week's directory
2. Write code following existing patterns
3. Add comprehensive tests
4. Run `cargo test` to verify
5. Run `cargo clippy` to check code quality
6. Format with `cargo fmt`
7. Update README with new functionality
8. Commit changes

### Working with Polynomial Plotter
The polynomial plotter exists in two locations:
- `/polynomial-plotter.html` - Standalone version
- `/code/interactiva_panel/polynomial-plotter.html` - Interactive panel version
- `/cryptography/polynomial-plotter.html` - Cryptography section version

All versions share similar logic for visualizing polynomials and ZK-SNARK concepts.

### Photo Gallery Template Structure
Individual gallery pages follow the HTML5 UP Multiverse template:

**HTML Structure:**
```html
<div id="main">
    <article class="thumb">
        <a href="category/photo.jpg" class="image">
            <img src="category/photo.jpg" alt="" loading="lazy" />
        </a>
    </article>
    <!-- More photos... -->
</div>
```

**Key CSS Features:**
- Flexbox container with `flex-wrap: wrap`
- `.thumb` elements with percentage-based widths (25%, 33.33%, 50%, 100%)
- Viewport-based heights: `calc(40vh - 2em)`
- Responsive breakpoints for different screen sizes

**JavaScript:**
- jQuery Poptrox initialized on `#main .thumb a` selector (via CDN)
- Handles modal popup, navigation arrows, and ESC key to close
- jQuery util.js provides browser compatibility helpers

**Regenerating Galleries:**
To regenerate all gallery HTML files after adding photos:
1. Use the Python generator script (create from template)
2. Manually edit individual HTML files to add `<article class="thumb">` blocks
3. Ensure photo paths use relative URLs from `/photos/` directory

## Important Notes

- **Multi-purpose Repository**: This serves as both a personal portfolio website AND a learning workspace for ZK proofs
- **No Build Process for Web**: Static HTML files are served directly; changes appear immediately after push
- **Rust Projects Are Independent**: Each week's ZK learning project has its own Cargo.toml and dependencies
- **Test-Driven Learning**: The ZK learning path emphasizes writing tests to verify understanding of cryptographic concepts
- **GitHub Pages Multi-Branch**: Always push to main, master, and gh-pages branches to ensure deployment works
- **HTML5 UP Templates**: Photo galleries use the Multiverse template; main site uses Ethereal template
- **Photo Gallery Structure**: Individual category pages in `/photos/*.html` use jQuery Poptrox (via CDN) for modal popup functionality

## Legacy Content Notes

- The `/content/` folder was removed (old Pelican setup); the site now uses direct `/photos/` and `/paint/` folders
- No Pelican build process; this is a pure static HTML site
- All HTML files are served directly from the root directory
- Photo galleries updated (Feb 2025) to use HTML5 UP Multiverse template with SimpleLightbox (via CDN)
- Photo galleries reverted (Feb 2026) to pure HTML5 UP Multiverse template with jQuery Poptrox modal popup
- Individual category pages (`photos/*.html`) replaced the previous single-page gallery approach
- Photo gallery index updated (Feb 2026) to show simple photo count text instead of progress bars