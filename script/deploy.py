#!/usr/bin/env python3
"""
Simple deployment script
1. Generate photo galleries
2. Build with Pelican
3. Deploy to GitHub Pages
"""

import subprocess
import sys
from pathlib import Path

def run_command(cmd, cwd=None):
    """Run a command and return success"""
    try:
        result = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True, text=True)
        if result.returncode != 0:
            print(f"Error: {result.stderr}")
            return False
        if result.stdout:
            print(result.stdout)
        return True
    except Exception as e:
        print(f"Exception: {e}")
        return False

def main():
    project_root = Path(__file__).parent.parent
    content_dir = project_root / "content"

    # 1. Generate galleries
    print("Generating photo galleries...")
    if not run_command("python script/generate_galleries.py --categories-dir content/content/photos/manual-categories --output-dir content/content/photography --folder-indexes --indexes-output output/photos/manual-categories", project_root):
        print("Failed to generate galleries")
        sys.exit(1)

    # 2. Build site
    print("\nBuilding site with Pelican...")
    if not run_command("pelican content -s publishconf.py -o ../output", content_dir):
        print("Failed to build site")
        sys.exit(1)

    # 3. Copy photo categories
    print("\nCopying photo directories...")
    if not run_command("cp -r content/content/photos/manual-categories output/photos/", project_root):
        print("Failed to copy photos")
        sys.exit(1)

    # 4. Deploy to GitHub Pages
    print("\nDeploying to GitHub Pages...")
    if not run_command("ghp-import output -b gh-pages -p -f", project_root):
        print("Failed to deploy")
        sys.exit(1)

    print("\n✅ Deployment complete!")

if __name__ == "__main__":
    main()