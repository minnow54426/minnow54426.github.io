# Personal Portfolio Website

A personal portfolio website hosted on GitHub Pages, featuring photography, blockchain and cryptography projects, and creative works.

## Features

- **Photography Gallery**: 15 photo collections with 247 images from travels across China
- **Code & Cryptography**: [blockchain_from_scratch](https://github.com/minnow54426/blockchain_from_scratch) — a blockchain built in Rust from first principles
- **Creative Works**: Music projects and watercolor video gallery
- **NTT-GPU**: [CKKS From Scratch](systems/ntt-gpu/index.html) — interactive
  explanations, implementation surveys, plans, and a daily engineering journal.

## Live Site

[https://minnow54426.github.io/](https://minnow54426.github.io/)

## Local Development

```bash
# Clone the repository
git clone https://github.com/minnow54426/minnow54426.github.io.git
cd minnow54426.github.io

# Start local development server
python -m http.server 8001

# View in browser
# Main page: http://localhost:8001/
# Photo gallery: http://localhost:8001/photo-gallery.html
```

## Project Structure

### NTT-GPU HTML documentation

The **Systems from Scratch** section opens the explicit HTML entry page
`systems/ntt-gpu/index.html`. It can also be opened directly as a local file;
its navigation, search, and math assets are bundled alongside it.

The pages are generated from the separate NTT-GPU repository. To refresh them,
run from that repository:

```sh
.venv/bin/python scripts/export_website.py --website-root /path/to/website
```

Serve this website root on port 8001 and open
`http://127.0.0.1:8001/systems/ntt-gpu/index.html`.
Edit NTT-GPU source documents rather than the generated website copy.

```
/
├── index.html              # Main landing page
├── photo-gallery.html      # Photo category index
├── paint.html              # Watercolor video gallery
├── music.html              # Music projects
├── assets/                 # CSS and JavaScript assets
├── photos/                 # Photography collection (15 categories)
└── paint/                  # Art and video collection
```

## Technology Stack

### Frontend
- HTML5/CSS3
- Restored Sunlit Paper design (`assets/css/theme.css`)
- Locally hosted Anton and Space Grotesk (licenses in `assets/fonts/`)
- Native dialog photo viewer with arrow-key navigation and focus restoration
- Keyboard-accessible painting folders and playback controls
- Font Awesome
- GitHub Pages

## Design and language

All published text and accessible labels use English. On November 16, 2026,
the historical visual design was restored from Git: the portfolio uses Sunlit
Paper colors, local Anton/Space Grotesk fonts, photo-filled lettering and
edge-to-edge photography. The NTT notebook shares the same palette and typography,
with readable math/code, section navigation and an expanded directory. Current research, GPU
results, native photo viewing and accessible media controls are preserved.
The current website has 59 HTML pages: 19 portfolio/collection pages and
40 generated notebook pages. Content updates should preserve this design.

## Deployment

The `Deploy to GitHub Pages` workflow publishes pushes to `main` and can also
be started manually from Actions.

In **Settings → Pages → Build and deployment**, set **Source → GitHub Actions**.
Branch-based publishing must be disabled: running the legacy branch publisher
alongside this workflow creates competing Pages deployments. The workflow
checks this setting before uploading and gives an actionable error if it drifts.

The workflow uses Node 24 actions, uploads the complete static website, and
deploys the resulting artifact. Publishing is complete only after the deployment
succeeds and the public page shows the new revision.

```bash
# Make changes locally
# Test with python -m http.server 8001

# Commit and push
git add .
git commit -m "Description of changes"
git push origin main

# Check the Deploy to GitHub Pages run, then verify https://minnow54426.github.io/
```

## License

This project uses open-source templates and libraries. See individual component directories for specific licenses.

## Author

Created and maintained by [wonderonpathlesspath](https://github.com/minnow54426/minnow54426.github.io)
