# Personal Portfolio Website

A personal portfolio website hosted on GitHub Pages, featuring photography, code projects, and interactive cryptography visualizations.

## Features

- **Photography Gallery**: 12 curated photo categories with 200+ images from travels across China
- **Code Projects**: Interactive visualizations and Zero Knowledge Proof implementations
- **Cryptography Tools**: Interactive polynomial plotter for understanding ZK-SNARK mathematics
- **Creative Works**: Music projects and watercolor video gallery

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
# Polynomial plotter: http://localhost:8001/cryptography/polynomial-plotter.html
```

## Project Structure

```
/
├── index.html              # Main landing page
├── photo-gallery.html      # Photo category index
├── paint.html              # Watercolor video gallery
├── code.html               # Code projects navigation
├── music.html              # Music projects
├── assets/                 # CSS and JavaScript assets
├── photos/                 # Photography collection (12 categories)
├── paint/                  # Art and video collection
├── code/                   # Code projects
│   └── groth16-demo/      # Groth16 ZK-SNARK implementation
└── cryptography/           # Interactive visualization tools
```

## Technology Stack

### Frontend
- HTML5/CSS3
- HTML5 UP templates (Ethereal, Multiverse)
- jQuery & jQuery Poptrox
- Font Awesome
- GitHub Pages

### Rust Projects
- Rust 2021 Edition
- Cargo Workspaces
- ARK Crypto Libraries (ark-ff, ark-ec, ark-bn254, ark-groth16)
- mdbook for documentation

## Deployment

The site is automatically deployed to GitHub Pages when changes are pushed to the `main` branch.

```bash
# Make changes locally
# Test with python -m http.server 8001

# Commit and push
git add .
git commit -m "Description of changes"
git push origin main

# Site updates in 1-2 minutes at https://minnow54426.github.io/
```

## License

This project uses open-source templates and libraries. See individual component directories for specific licenses.

## Author

Created and maintained by [wonderonpathlesspath](https://github.com/minnow54426/minnow54426.github.io)
