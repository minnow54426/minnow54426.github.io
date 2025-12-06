#!/usr/bin/env python3
"""
Script to deploy the website with photos
1. Organize existing photos using organize_photos.py
2. Generate photo galleries using generate_sub_galleries.py
3. Publish the website using Pelican
4. Deploy to GitHub Pages
"""

import subprocess
import os
import sys
from pathlib import Path


def organize_photos():
    """Organize existing photos using the organize_photos script"""
    script_dir = Path(__file__).parent
    organize_script = script_dir / "organize_photos.py"

    try:
        print("Organizing photos...")
        result = subprocess.run([sys.executable, str(organize_script)],
                              capture_output=True, text=True, cwd=script_dir)

        if result.returncode == 0:
            print("Successfully organized photos")
            if result.stdout:
                print(result.stdout)
            return True
        else:
            print(f"Error organizing photos: {result.stderr}")
            return False
    except Exception as e:
        print(f"Exception occurred during photo organization: {e}")
        return False


def generate_galleries():
    """Generate photo galleries using the generate_sub_galleries script"""
    script_dir = Path(__file__).parent
    gallery_script = script_dir / "generate_sub_galleries.py"

    try:
        print("Generating photo galleries...")
        result = subprocess.run([sys.executable, str(gallery_script)],
                              capture_output=True, text=True, cwd=script_dir)

        if result.returncode == 0:
            print("Successfully generated photo galleries")
            if result.stdout:
                print(result.stdout)
            return True
        else:
            print(f"Error generating galleries: {result.stderr}")
            return False
    except Exception as e:
        print(f"Exception occurred during gallery generation: {e}")
        return False


def publish_website():
    """Publish the website using Pelican"""
    content_dir = Path(__file__).parent.parent / "content"
    
    # Change to content directory
    original_dir = os.getcwd()
    os.chdir(content_dir)
    
    try:
        print("Publishing website...")
        # Run make publish
        result = subprocess.run(["make", "publish"], capture_output=True, text=True)
        
        if result.returncode == 0:
            print("Successfully published website")
            if result.stdout:
                print(result.stdout)
            return True
        else:
            print(f"Error publishing website: {result.stderr}")
            return False
    except Exception as e:
        print(f"Exception occurred during website publishing: {e}")
        return False
    finally:
        # Change back to original directory
        os.chdir(original_dir)


def deploy_to_github():
    """Deploy the website to GitHub Pages"""
    content_dir = Path(__file__).parent.parent / "content"
    
    # Change to content directory
    original_dir = os.getcwd()
    os.chdir(content_dir)
    
    try:
        print("Deploying to GitHub Pages...")
        # Run make github
        result = subprocess.run(["make", "github"], capture_output=True, text=True)
        
        if result.returncode == 0:
            print("Successfully deployed to GitHub Pages")
            if result.stdout:
                print(result.stdout)
            return True
        else:
            print(f"Error deploying to GitHub Pages: {result.stderr}")
            return False
    except Exception as e:
        print(f"Exception occurred during GitHub deployment: {e}")
        return False
    finally:
        # Change back to original directory
        os.chdir(original_dir)


def main():
    print("Starting deployment process with photos...")

    # Step 1: Organize existing photos
    if not organize_photos():
        print("Failed to organize photos. Aborting deployment.")
        return 1

    # Step 2: Generate photo galleries
    if not generate_galleries():
        print("Failed to generate galleries. Aborting deployment.")
        return 1

    # Step 3: Publish website
    if not publish_website():
        print("Failed to publish website. Aborting deployment.")
        return 1

    # Step 4: Deploy to GitHub Pages
    if not deploy_to_github():
        print("Failed to deploy to GitHub Pages.")
        return 1

    print("Deployment completed successfully!")
    return 0


if __name__ == "__main__":
    sys.exit(main())