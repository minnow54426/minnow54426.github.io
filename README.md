# Personal Portfolio Website

A personal portfolio website hosted on GitHub Pages, featuring photography, blockchain and cryptography projects, and creative works.

## Features

- **Photography Gallery**: 14 curated photo categories with 230+ images from travels across China
- **Code & Cryptography**: [blockchain_from_scratch](https://github.com/minnow54426/blockchain_from_scratch) — a blockchain built in Rust from first principles
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
```

## Project Structure

```
/
├── index.html              # Main landing page
├── photo-gallery.html      # Photo category index
├── paint.html              # Watercolor video gallery
├── music.html              # Music projects
├── assets/                 # CSS and JavaScript assets
├── photos/                 # Photography collection (14 categories)
└── paint/                  # Art and video collection
```

## Technology Stack

### Frontend
- HTML5/CSS3
- Sunlit Paper custom design system (`assets/css/theme.css`)
- jQuery & jQuery Poptrox
- Font Awesome
- GitHub Pages

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
