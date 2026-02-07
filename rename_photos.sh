#!/bin/bash
# Rename photos to y-m-d-001.jpg format

DATE="2025-02-07"

echo "=== Photo Renaming Script ==="
echo "Format: ${DATE}-001.jpg"
echo "This will rename all photos in each category folder"
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
  echo "Cancelled"
  exit 1
fi

# Process each category folder
for category_dir in photos/*/; do
  category=$(basename "$category_dir")
  echo "Processing: $category"

  # Find all image files and rename them
  counter=1
  find "$category_dir" -type f \( -name "*.jpg" -o -name "*.JPG" -o -name "*.jpeg" -o -name "*.png" \) | sort | while read oldfile; do
    # Get extension
    ext="${oldfile##*.}"
    ext=$(echo "$ext" | tr '[:upper:]' '[:lower:]')  # convert to lowercase

    # Create new filename with zero-padded counter
    newname=$(printf "%s/%s-%03d.%s" "$category_dir" "$DATE" "$counter" "$ext")

    # Rename
    mv "$oldfile" "$newname"
    echo "  $(basename "$oldfile") → $(basename "$newname")"

    ((counter++))
  done
  echo ""
done

echo "=== Renaming Complete ==="
