# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
This is a Pelican static site generator project for a personal blog/wbsite named "wonderonpathlesspath". Pelican is a Python-based static site generator that transforms plain text (Markdown or reStructuredText) into a static website.

## Directory Structure
- `content/` - Source content files (Markdown posts)
- `output/` - Generated static website files
- `pelicanconf.py` - Development configuration
- `publishconf.py` - Production configuration
- `tasks.py` - Custom development tasks using Invoke
- `Makefile` - Standard make commands for building and deployment

## Common Development Commands

### Building and Running Locally
- `make html` - Generate the website
- `make clean` - Remove generated files
- `make regenerate` - Regenerate files upon modification
- `make serve` - Serve site locally at http://localhost:8000
- `make devserver` - Serve and regenerate together (recommended for development)

### Using Invoke Tasks (Alternative to Make)
- `invoke build` - Build local version of site
- `invoke rebuild` - Build with delete switch
- `invoke regenerate` - Automatically regenerate site upon file modification
- `invoke serve` - Serve site locally
- `invoke reserve` - Build, then serve
- `invoke preview` - Build production version of site
- `invoke livereload` - Automatically reload browser tab upon file modification

### Production Deployment
- `make publish` - Generate using production settings
- `make github` - Deploy to GitHub Pages
- `invoke gh-pages` - Alternative for GitHub Pages deployment
- `invoke publish` - Publish to production via rsync

## Code Architecture
- Content is written in Markdown files in the `content/` directory
- Configuration is managed through `pelicanconf.py` (dev) and `publishconf.py` (prod)
- Custom tasks are implemented in `tasks.py` using the Invoke library
- The build process transforms Markdown content into static HTML/CSS/JS in the `output/` directory
- Themes and styling can be customized, with static assets in theme directories

## Key Configuration Files
1. `pelicanconf.py` - Development settings (empty SITEURL, no feeds)
2. `publishconf.py` - Production settings (HTTPS SITEURL, feed generation, DELETE_OUTPUT_DIRECTORY)
3. `tasks.py` - Custom development tasks and server functionality
4. `Makefile` - Standard build and deployment commands

When working with content:
- Add new posts as Markdown files in the `content/` directory
- Follow the metadata format (Title, Date, Category, Tags, Slug, Author, Summary)
- Use `make devserver` or `invoke livereload` for live development preview