# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Personal portfolio website with GitHub Pages, featuring photography, a Rust blockchain learning project, and creative works (watercolor, music).

## Key Commands

### Local Development
```bash
# Start local development server (from repository root)
python -m http.server 8001

# View local site
# Main page:         http://localhost:8001/
# Photo gallery:     http://localhost:8001/photo-gallery.html
# Paint gallery:     http://localhost:8001/paint.html
# Music:             http://localhost:8001/music.html
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

Co-Authored-By: Claude <noreply@anthropic.com>"

# Push to main branch (GitHub Pages deployment)
git push origin main
```

**IMPORTANT**: Only push to the main branch. GitHub Pages uses main as the default branch.

**Proxy gotcha**: `git config --global http.proxy` is set to `http://127.0.0.1:7890` (Clash/V2Ray) and is frequently down, which makes `git fetch`/`push` fail ("Failed to connect to 127.0.0.1 port 7890"). GitHub is reachable directly — bypass the proxy per-command:
```bash
git -c http.proxy= -c https.proxy= push origin main   # or fetch/pull
HTTPS_PROXY= HTTP_PROXY= gh run list                   # or curl
```

### Photo Management

```bash
# Add new photos to existing category
cp /path/to/new/photos/*.jpg "photos/shang hai/"

# Check file sizes before committing
ls -lh photos/shang\ hai/

# For new categories:
mkdir "photos/new category"
cp /path/to/photos/*.jpg "photos/new category/"
# Create new category HTML file following an existing one in photos/ (e.g. photos/shang-hai.html)
# Add a category card to photo-gallery.html
```

### Video Management

```bash
# Standard compression for new videos (recommended)
ffmpeg -i original.mov -c:v libx264 -preset medium -crf 28 -movflags +faststart compressed.mp4

# Aggressive compression for large videos to get under 10MB
# Use higher CRF values (32-40) for larger files
ffmpeg -i large_video.mp4 -c:v libx264 -preset medium -crf 35 -movflags +faststart output.mp4

# Check video properties
ffprobe -v quiet -print_format json -show_format -show_streams video.mp4

# Check file sizes (must be under 10MB for GitHub Pages)
ls -lh paint/water_color/
ls -lh paint/learning/
ls -lh paint/creat/

# Force add videos to git (ignored by .gitignore)
git add -f paint/learning/*.mp4 paint/creat/*.mp4
```

## Directory Structure

```
/
├── index.html              # Landing page — Sunlit Paper theme (masked hero + editorial plates)
├── photo-gallery.html      # Photography category index (14 category cards)
├── paint.html              # Paint/watercolor video gallery
├── music.html              # Music page (coming soon)
│
├── assets/
│   ├── css/               # theme.css = active design system; fontawesome-all.min.css
│   ├── js/                # jQuery, jquery.poptrox, Ethereal utils (all loaded by photo gallery pages)
│   └── webfonts/          # Font Awesome webfonts
│
├── js/
│   └── theme.js           # Scroll-reveal, lazy video, nav-spy (vanilla, reduced-motion aware)
│
├── photos/                 # Photography collection (14 categories, 234 JPGs)
│   ├── shang hai/         # 98
│   ├── animals/           # 23
│   ├── jiu zhai gou/      # 20
│   ├── zhang jia jie/     # 13
│   ├── hu pao gong yuan/  # 14
│   ├── qing dao/          # 15
│   ├── nan xun gu zhen/   # 10
│   ├── qian dao hu/       # 10
│   ├── ao men/            # 7
│   ├── zhu hai/           # 3
│   ├── on road/           # 4
│   ├── hang zhou/         # 5
│   ├── wu xi/             # 10
│   ├── others/            # 2
│   │
│   ├── gallery.css        # Shared gallery styles + Poptrox lightbox (Sunlit Paper)
│   ├── gallery.js         # jQuery Poptrox initialization
│   └── *.html             # Individual category pages (14 files)
│
├── paint/                  # Watercolor videos (27 total, all <10MB)
│   ├── water_color/       # 12 videos
│   ├── learning/          # 9 videos
│   └── creat/             # 6 videos
│
├── .nojekyll              # Disable Jekyll processing for GitHub Pages
├── .gitignore             # Git ignore rules
├── README.md              # Project documentation
└── CLAUDE.md              # This file
```

## Website Architecture

### Theme System — Sunlit Paper

The entire site uses one custom design system: **`assets/css/theme.css`**. The old HTML5 UP Ethereal/Multiverse stylesheets remain on disk but are **no longer linked**.

**Palette** (CSS custom properties on `:root`):
- `--bg #efe4cd` (warm paper), `--surface #e3d4b3` (ivory), `--cream #2b1d10` (espresso ink — primary text)
- `--accent #b5421a` (burnt orange), `--accent2 #1f5d3e` (forest green)
- `--rule`, `--muted` for borders/secondary text

**Typography**: **Anton** (display, Google Fonts) for headlines and masked wordmarks; **Space Grotesk** for body/UI; Font Awesome for icons.

**Signature technique — photo-into-type mask**: the `.mask` class uses `background-clip: text` + `-webkit-text-fill-color: transparent` with a `--mask-img` CSS variable to render a photograph *inside* the letterforms (hero wordmark, plate word marks, footer mark). Always provide a solid fallback color (set in `.mask`) so text remains legible if the image fails.
  - **Gotcha**: relative `url()` in `theme.css` resolves against `assets/css/`, so mask image paths must be **root-relative** (`/photos/...`) — relative paths 404.
  - **Gotcha**: a `transform` on a descendant of a `.mask` element, or a `filter` on the element itself, silently breaks `background-clip: text` in Chromium (the photo stops painting through the glyphs → invisible text). Don't add `transform`/`filter` to masked elements.

**Behavior** (`js/theme.js`, vanilla, dependency-free):
- `IntersectionObserver` scroll-reveal (`.reveal` → `.is-in`), with `data-delay` stagger.
- Topbar nav-spy (highlights the current section's anchor).
- Lazy video: `video[data-autoplay]` plays when scrolled into view, pauses when leaving; skipped entirely under `prefers-reduced-motion`.
- All motion respects `prefers-reduced-motion`.

### Main Page (`index.html`)

- **Hero**: masked `WONDER ON / PATHLESS PATH` wordmark (photo through the letters), tagline, scroll cue.
- **Manifesto**: large-type statement — "Capturing moments. Building systems. Exploring cryptography."
- **Four editorial plates** (alternating left/right layout), each with a masked word mark + outlined numeral:
  1. **Photography** (`PHOTO`) → `photo-gallery.html`
  2. **Code** (`CODE`, combined code + cryptography) → blockchain_from_scratch (GitHub)
  3. **Paint** — featured watercolor video (lazy, autoplay-in-view) → `paint.html`
  4. **Music** (`MUSIC`) → `music.html` (coming soon)
- **Footer**: masked `WP` mark + nav links.

### Photo Gallery System

**Category index** (`photo-gallery.html`): 14 `.cat-card` entries (representative photo + name + count) in a responsive grid, each linking to its category page.

**Category pages** (`photos/*.html`, 14 files): responsive thumbnail grid of `<article class="thumb">` blocks; jQuery Poptrox lightbox.
- Shared styles: `photos/gallery.css` (Sunlit Paper, including the Poptrox popup `.closer` / `.nav-*` / backdrop).
- Lightbox config: `photos/gallery.js` (`usePopupDefaultStyling: false` → all popup CSS lives in `gallery.css`). Poptrox plugin at `assets/js/jquery.poptrox.min.js`.
- Responsive: 4 columns (≥1680px) → 3 (≥1280) → 2 (≥768) → 1 (mobile); thumb height `calc(40vh - 2em)`.
- **Lightbox flash fix**: Poptrox's completion callback hides `.pic` and fades it back in at the end of the expand, which flashes. `gallery.css` sets `.poptrox-popup .pic { opacity: 1 !important }` to suppress it — keep this override.

### Code Projects

There is **no `code.html` index** and **no local `code/` directory**. The homepage "Code" plate (combined code + cryptography) links to a single external resource: the **blockchain_from_scratch** GitHub repo.

### Paint Gallery (`paint.html`)

- **Three-folder structure** with expandable/collapsible sections (vanilla JS; all gallery class names are depended on by the inline script — preserve them when editing).
- **Water Color** (12 videos): Christmas Snowman, Single Leaf, Mountain, Leaf on Water, Flower, Autumn Leave, Rose, Peach, Cherry Blossoms, Swan, Flower Bed, Whale
- **Learning** (9 videos): Bee, Camping, Character Avatar, Chicken, Cloud Castle, Crane, Flamingo, Hummingbird, Oasis
- **Creative** (6 videos): Cherry Blossoms, Cloud, Ghost, Mountain, Snow, Son of Light

**Features**:
- Custom video controls (play/pause, progress bar, speed control, loop toggle)
- Hover preview with autoplay
- Picture-in-Picture support
- Fullscreen mode
- Responsive grid layout with staggered animations
- All videos compressed under 10MB for GitHub Pages (total ~50MB)

## Development Workflow

### Adding New Web Content

1. Create the HTML file in the root (or relevant directory).
2. Link `assets/css/theme.css` + the Google Fonts (Anton + Space Grotesk); Font Awesome optional. Use the shared chrome classes: `.topbar`, `.page-hero`, `.shell`, `.footer`, `.back-link`.
3. Add navigation links where relevant (topbar nav, homepage plates, footer).
4. Test locally with `python -m http.server 8001`.
5. Commit and push to `main`.

### Adding New Photos

1. Copy photos to the appropriate category directory under `photos/`.
2. Add `<article class="thumb">` blocks to the category HTML file (see "Regenerating Photo Galleries" below).
3. Add a `.cat-card` to `photo-gallery.html` (with a representative photo + count).
4. Test locally.
5. Commit and push.

**Photo File Format**:
- Use EXIF-based naming: `YYYY-MM-DD-###.jpg`
- Example: `2024-08-17-001.jpg`, `2024-09-14-008.jpg`
- Keep files under 5MB each

### Adding New Videos

1. **Compress with FFmpeg** (CRF 28-32 for normal, 35-40 for large files):
   ```bash
   # For videos under 50MB
   ffmpeg -i original.mov -c:v libx264 -preset medium -crf 28 -movflags +faststart output.mp4

   # For larger videos to get under 10MB
   ffmpeg -i large_video.mp4 -c:v libx264 -preset medium -crf 35 -movflags +faststart output.mp4
   ```

2. **Copy to appropriate folder**:
   - `paint/water_color/` for basic watercolor videos
   - `paint/learning/` for tutorial/practice videos
   - `paint/creat/` for creative/experimental videos

3. **Add to `paint.html`**:
   - Add folder HTML section (if new category)
   - Add video data to `folderVideos` object in the inline JavaScript
   - Initialize gallery with `loadGallery('folder-id')`

4. **Force add to git** (videos ignored by .gitignore):
   ```bash
   git add -f paint/folder_name/*.mp4
   ```

5. **Test playback locally** and commit

**Important**: Videos must be under 10MB each for GitHub Pages. Use higher CRF values (35-40) for aggressive compression if needed.

### Regenerating Photo Galleries

If adding many photos at once, generate the `<article class="thumb">` blocks with a Python script:

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

- **Static Site**: No build step; HTML/CSS/JS served directly.
- **Theme**: single custom design system in `assets/css/theme.css` (Sunlit Paper). Legacy Ethereal assets (main.css, custom.css, noscript.css, `assets/sass/`, `images/`) have been removed from the repo.
- **Masked type**: `.mask` uses `background-clip: text`. Mask image paths must be **root-relative**; do **not** put `transform`/`filter` on masked elements (breaks the clip in Chromium → invisible text).
- **GitHub Pages**: `.nojekyll` disables Jekyll processing; deploys from `main` via GitHub Actions.
- **Git proxy**: `http.proxy` = `127.0.0.1:7890` (Clash) and is often down — bypass it for git/gh/curl to GitHub (see Git Workflow above). GitHub is reachable directly.
- **Video File Sizes**: All 27 paint videos compressed under 10MB each (~50MB total); `.mp4` is gitignored — use `git add -f`.
- **GitHub Actions**: Automatic deployment on push to `main`; workflow uploads the entire repository (`path: '.'`).
- **Git Worktrees**: `.worktrees/` directory contains isolated development branches (ignore in normal work).

## Tech Stack

### Web Frontend
- **Sunlit Paper theme** — custom design system (`assets/css/theme.css`); `background-clip: text` masked photography.
- **Anton + Space Grotesk** — display + body fonts (Google Fonts).
- **Vanilla JS** — `js/theme.js` (scroll-reveal, lazy video, nav-spy); no framework.
- **jQuery + jquery.poptrox** — photo gallery lightbox only (`photos/*.html`).
- **Font Awesome** — icons.
- **GitHub Pages** — static hosting.

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

Co-Authored-By: Claude <noreply@anthropic.com>"

# 4. Push to main branch (bypass the proxy if it's down)
git -c http.proxy= -c https.proxy= push origin main

# 5. Wait ~1 minute for GitHub Pages to rebuild
# 6. Verify live site (curl with proxy bypassed if needed)
HTTPS_PROXY= HTTP_PROXY= curl -s https://minnow54426.github.io/ | head
```

### Common Deployment Issues

1. **Files not appearing**: Check `.nojekyll` file exists (not using Jekyll)
2. **Large file push fails**: GitHub 100MB limit - compress or remove large files
3. **Changes not visible**: Clear browser cache, wait longer for deployment
4. **Wrong branch deployed**: Verify GitHub Pages repository settings
5. **GitHub Actions fails**: Check that all videos are under 10MB
6. **`Failed to connect to 127.0.0.1 port 7890`**: the configured proxy (Clash) is down — bypass it with `git -c http.proxy= -c https.proxy=`.

### GitHub Actions Workflow

The site uses GitHub Actions for automatic deployment to GitHub Pages.

**Workflow file**: `.github/workflows/deploy.yml`

**Triggers**:
- Push to `main` branch
- Manual workflow dispatch

**Uploaded**: the workflow uploads `path: '.'` (the entire repository), so every file is included — HTML pages, `assets/`, `js/`, `photos/` (14 categories), and `paint/` videos.

**Requirements**:
- All video files must be under 10MB
- Videos are tracked in git despite the `.gitignore` `*.mp4` rule (force added with `git add -f`)
- Deployment typically completes in ~1 minute

### File Size Guidelines
- **Images**: Under 5MB each
- **Videos**: Under 10MB each (strict requirement for GitHub Pages deployment)
  - Use CRF 28-32 for normal compression
  - Use CRF 35-40 for aggressive compression of large files
  - All current paint videos: 1-9MB each (27 videos total, ~50MB)
- **Total repository**: Under 1GB recommended (currently ~500MB with all videos)
