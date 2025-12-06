#!/bin/bash
# Script to deploy photos in smaller batches

REPO_DIR="/Users/boycrypt/code/python/website/content"
OUTPUT_DIR="$REPO_DIR/output"
PHOTOS_DIR="$OUTPUT_DIR/photos"
TEMP_DEPLOY_DIR="$REPO_DIR/temp_photos_deploy"

# Create temporary deployment directory
mkdir -p "$TEMP_DEPLOY_DIR"

# Copy photos directory to temporary deployment
cp -r "$PHOTOS_DIR" "$TEMP_DEPLOY_DIR/"

# Deploy with ghp-import
echo "Deploying photos to GitHub Pages..."
cd "$REPO_DIR"
ghp-import "$TEMP_DEPLOY_DIR" -m "Deploy photos for gallery" -b gh-pages --force

# Push to GitHub
echo "Pushing to GitHub..."
cd "$REPO_DIR"
git push origin gh-pages

# Clean up
rm -rf "$TEMP_DEPLOY_DIR"

echo "Photos deployment complete!"