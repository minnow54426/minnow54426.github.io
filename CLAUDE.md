# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Personal portfolio website with GitHub Pages, featuring photography, code projects (Zero Knowledge Proof learning and Groth16 implementation), and interactive cryptography visualizations.

## Key Commands

### Local Development
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

### Rust Projects

#### Groth16 Demo (Code Project)
```bash
# Navigate to project
cd code/groth16-demo

# Build workspace
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

# Generate documentation
cargo doc --open

# Run benchmarks
cargo bench

# Build mdbook documentation
mdbook build book
```

#### Individual Groth16 Crates
The Groth16 project is a workspace with multiple crates:
- `crates/math` - Field operations, pairings, polynomials
- `crates/r1cs` - Rank-1 Constraint System
- `crates/qap` - Quadratic Arithmetic Programs
- `crates/groth16` - Proof generation and verification
- `crates/circuits` - Circuit implementations (multiplier, hash preimage, merkle, cubic)

Each crate can be worked on individually:
```bash
cd code/groth16-demo/crates/groth16
cargo test
```

### Git Workflow
```bash
# Check status and review changes
git status
git diff

# Stage and commit
git add .
git commit -m "Brief description

Detailed explanation.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"

# Push to all branches (GitHub Pages deployment)
git push origin main
git push origin master
git push origin gh-pages
```

**IMPORTANT**: Always push to main, master, and gh-pages branches. GitHub Pages may use any of these as the default branch.

### Photo Management

```bash
# Add new photos to existing category
cp /path/to/new/photos/*.jpg "photos/shang hai/"

# Check file sizes before committing
ls -lh photos/shang\ hai/

# For new categories:
mkdir "photos/new category"
cp /path/to/photos/*.jpg "photos/new category/"
# Create new category HTML file following the template
# Update photo-gallery.html to include new category link
```

### Video Management

```bash
# Standard compression (recommended)
ffmpeg -i original.mov -c:v libx264 -preset medium -crf 28 -movflags +faststart compressed.mp4

# Check video properties
ffprobe -v quiet -print_format json -show_format -show_streams video.mp4

# Check file sizes (must be under 100MB for GitHub)
ls -lh paint/water_color/
```

## Directory Structure

```
/
├── index.html              # Main landing page (HTML5 UP Ethereal template)
├── photo-gallery.html      # Photo category index page
├── paint.html              # Paint/watercolor video gallery
├── code.html               # Code projects file tree navigation
├── music.html              # Music projects page
├── cryptography.html       # Cryptography learning and tools
├── polynomial-plotter.html # Standalone polynomial visualization
│
├── assets/                 # HTML5 UP Ethereal template assets
│   ├── css/               # Stylesheets (main.css, custom.css)
│   ├── js/                # JavaScript (jQuery, browser breakpoints)
│   ├── sass/              # SASS source files
│   └── webfonts/          # Font Awesome webfonts
│
├── js/                     # Custom JavaScript
│   └── home.js            # Homepage photo gallery preview
│
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
│   │
│   ├── gallery.css        # Shared gallery styles (Multiverse template)
│   ├── gallery.js         # jQuery Poptrox initialization
│   └── *.html             # Individual category pages (12 files)
│
├── paint/                  # Art and video collection
│   └── water_color/       # Watercolor videos (12 videos, MP4 format)
│
├── code/                   # Code projects directory
│   ├── christmas_tree/    # Christmas tree visualization
│   ├── interactiva_panel/ # Interactive panel with polynomial plotter
│   └── groth16-demo/      # Groth16 ZK-SNARK implementation (workspace)
│       ├── crates/        # Workspace crates (math, r1cs, qap, groth16, circuits)
│       ├── book/          # mdbook interactive documentation
│       ├── docs/          # Additional documentation
│       └── Cargo.toml     # Workspace configuration
│
├── cryptography/           # Cryptography interactive tools
│   ├── app.js             # Polynomial plotter logic
│   ├── polynomial-plotter.html
│   └── styles.css
│
├── .nojekyll              # Disable Jekyll processing for GitHub Pages
├── .gitignore             # Git ignore rules
├── README.md              # Project documentation
└── CLAUDE.md              # This file
```

## Website Architecture

### Template System

The site uses **HTML5 UP templates**:

1. **Ethereal Template** (`assets/css/main.css`)
   - Used for: `index.html`, `photo-gallery.html`, `cryptography.html`, `code.html`
   - Features: Vertical scrolling, panel-based layout, gradient backgrounds
   - Custom overrides in: `assets/css/custom.css`

2. **Multiverse Template** (`photos/gallery.css`)
   - Used for: Individual photo category pages in `/photos/*.html`
   - Features: Flexbox grid, modal popup (jQuery Poptrox), responsive columns
   - Key characteristic: Dark theme with `#1a1a1a` background

### Main Page Structure (`index.html`)

- **Hero Panel**: Full-screen gradient header with site title and navigation
- **Photography Panel**: Photo gallery preview (8 photos from different categories)
- **Code Projects Panel**: Cards for ZK learning, Christmas Tree, Interactiva Panel, Groth16 Demo
- **Cryptography Panel**: Links to interactive polynomial plotter
- **Creative Works Panel**: Music and paint gallery links

### Photo Gallery System

**Category Index** (`photo-gallery.html`):
- 12 category cards with photo counts (e.g., "62 photos")
- Links to individual category pages
- Uses Ethereal template styling

**Individual Category Pages** (`photos/*.html`):
- HTML5 UP Multiverse template with jQuery Poptrox
- Modal popup: Click photo → full-size view with close button
- Next/Prev navigation arrows between photos
- ESC key or click outside to close
- Responsive grid: 4 columns (desktop) → 1 column (mobile)
- Viewport-based photo height: `calc(40vh - 2em)`

**Shared Gallery Resources**:
- `photos/gallery.css` - Multiverse template styles
- `photos/gallery.js` - jQuery Poptrox initialization
- `assets/css/main.css` - HTML5 UP Multiverse (linked as `../assets/css/main.css`)

### Code Projects Page (`code.html`)

- File tree navigation with expandable/collapsible folders
- Projects:
  - `christmas_tree/` - Interactive visualization
  - `interactiva_panel/` - Polynomial plotter
  - `groth16-demo/` - Complete Groth16 implementation with mdbook
- JavaScript for folder expand/collapse behavior

### Groth16 Demo Project

**Workspace Structure**:
```
code/groth16-demo/
├── Cargo.toml           # Workspace configuration
├── book/                # mdbook interactive documentation
│   └── book/           # Built HTML output
├── crates/
│   ├── math/           # Field wrapper, pairings, polynomials
│   ├── r1cs/           # R1CS constraints and witnesses
│   ├── qap/            # QAP transformation and divisibility
│   ├── groth16/        # Trusted setup, proofs, verification
│   └── circuits/       # Example circuits (multiplier, hash_preimage, merkle, cubic)
└── examples/           # Binary demos (multiplier_demo, hash_preimage_demo, merkle_demo)
```

**Key Dependencies** (from `Cargo.toml`):
- `ark-*` crates (ark-ff, ark-ec, ark-bn254, ark-poly, ark-groth16, etc.)
- `serde` + `bincode` for serialization
- `anyhow` + `thiserror` for error handling
- `proptest` for property-based testing

**Architecture Patterns**:
1. **Workspace Pattern**: Multiple crates in one Cargo workspace
2. **Type Safety**: Field wrappers, newtype patterns for cryptographic data
3. **Modular Design**: Clear separation between math, R1CS, QAP, and Groth16 layers
4. **Documentation**: mdbook for interactive learning materials
5. **Testing**: Comprehensive unit tests and property-based tests

### Paint Gallery (`paint.html`)

- Folder-based navigation with 12 watercolor videos
- Responsive video grid with hover effects
- All videos optimized under 10MB for GitHub Pages
- Video categories: Christmas Snowman, Single Leaf, Mountain, Leaf on Water, Flower, Autumn Leave, Rose, Peach, Cherry Blossoms, Swan, Flower Bed, Whale

## Development Workflow

### Adding New Web Content

1. Create HTML file in root directory
2. Add appropriate CSS links (Ethereal or Multiverse template)
3. Add navigation links in `index.html` if needed
4. Test locally with `python -m http.server 8001`
5. Commit and push to all branches

### Adding New Photos

1. Copy photos to appropriate category directory
2. Add `<article class="thumb">` blocks to category HTML file
3. Update photo count in `photo-gallery.html`
4. Test locally
5. Commit and push

**Photo File Format**:
- Use EXIF-based naming: `YYYY-MM-DD-###.jpg`
- Example: `2024-08-17-001.jpg`, `2024-09-14-008.jpg`
- Keep files under 5MB each

### Adding New Videos

1. Compress with FFmpeg: `ffmpeg -i original.mov -c:v libx264 -preset medium -crf 28 -movflags +faststart output.mp4`
2. Copy to `paint/water_color/`
3. Add to `paint.html` video list
4. Test playback locally
5. Commit and push

**Important**: Videos must be under 100MB (GitHub limit), aim for under 10MB

### Working with Groth16 Code

1. Navigate to specific crate: `cd code/groth16-demo/crates/groth16`
2. Write code following existing patterns
3. Add comprehensive tests
4. Run `cargo test` to verify
5. Run `cargo clippy` for linting
6. Format with `cargo fmt`
7. Update documentation if needed
8. Build mdbook: `mdbook build book` (if documentation changed)
9. Commit changes

### Regenerating Photo Galleries

If adding many photos at once, you can use a Python script:

```python
import os
photo_dir = "shang hai"
photos = sorted([f for f in os.listdir(photo_dir) if f.lower().endswith('.jpg')])
for photo in photos:
    print(f'''			<article class="thumb">
				<a href="{photo_dir}/{photo}" class="image">
					<img src="{photo_dir}/{photo}" alt="" />
				</a>
			</article>''')
```

Then paste the output into the appropriate category HTML file.

## Important Notes

- **Static Site**: No build process for web content; HTML files served directly
- **GitHub Pages**: Uses `.nojekyll` to disable Jekyll processing
- **Multi-Branch Deployment**: Always push to main, master, and gh-pages branches
- **Template Assets**: HTML5 UP templates in `assets/` (Ethereal) and `photos/` (Multiverse)
- **Photo Gallery Dark Theme**: Individual category pages use dark background (`#1a1a1a`)
- **Groth16 Workspace**: Multi-crate Rust project requiring careful dependency management
- **Video File Sizes**: Must be under 100MB (GitHub hard limit), aim for under 10MB
- **Git Worktrees**: `.worktrees/` directory contains isolated development branches (ignore in normal work)

## Tech Stack

### Web Frontend
- **HTML5/CSS3** - Modern web standards
- **HTML5 UP Templates** - Ethereal (main site), Multiverse (photo galleries)
- **jQuery** - DOM manipulation and interactions
- **jQuery Poptrox** - Modal popup for photo galleries (via CDN in Multiverse template)
- **Font Awesome** - Icon system
- **GitHub Pages** - Static hosting

### Rust Projects
- **Rust 2021 Edition** - Modern Rust
- **Cargo Workspaces** - Multi-crate projects
- **ARK Crypto Libraries** (ark-ff, ark-ec, ark-bn254, ark-groth16, etc.)
- **mdbook** - Interactive documentation
- **serde/bincode** - Serialization
- **anyhow/thiserror** - Error handling
- **proptest** - Property-based testing

## Deployment

### Before Every Deployment
```bash
# 1. Local testing
python -m http.server 8001
# Test all pages and features in browser

# 2. Review changes
git status
git diff

# 3. Commit
git add .
git commit -m "Description

Details.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"

# 4. Push to all branches
git push origin main
git push origin master
git push origin gh-pages

# 5. Wait 1-2 minutes for GitHub Pages to rebuild
# 6. Verify live site
```

### Common Deployment Issues

1. **Files not appearing**: Check `.nojekyll` file exists (not using Jekyll)
2. **Large file push fails**: GitHub 100MB limit - compress or remove large files
3. **Changes not visible**: Clear browser cache, wait longer for deployment
4. **Wrong branch deployed**: Verify GitHub Pages repository settings

### File Size Guidelines
- **Images**: Under 5MB each
- **Videos**: Under 10MB each (GitHub limit is 100MB)
- **Total repository**: Under 1GB recommended
