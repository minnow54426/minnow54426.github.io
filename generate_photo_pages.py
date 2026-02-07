#!/usr/bin/env python3
"""
Generate individual photo category pages using Multiverse template style
"""

import os

# Photo categories with their display names and descriptions
categories = {
    'shang hai': {
        'name_en': 'Shanghai',
        'name_zh': '上海',
        'icon': 'fa-city',
        'description': 'Urban landscapes and city life from the Pearl of the Orient'
    },
    'jiu zhai gou': {
        'name_en': 'Jiu Zhai Gou',
        'name_zh': '九寨沟',
        'icon': 'fa-mountain',
        'description': 'Stunning valley scenery with crystal clear lakes and waterfalls'
    },
    'zhang jia jie': {
        'name_en': 'Zhang Jia Jie',
        'name_zh': '张家界',
        'icon': 'fa-mountain',
        'description': 'Dramatic sandstone pillars and breathtaking mountain vistas'
    },
    'hu pao gong yuan': {
        'name_en': 'Hu Pao Gong Yuan',
        'name_zh': '虎跑公园',
        'icon': 'fa-tree',
        'description': 'Serene park scenes with spring blossoms and peaceful paths'
    },
    'qing dao': {
        'name_en': 'Qing Dao',
        'name_zh': '青岛',
        'icon': 'fa-water',
        'description': 'Coastal beauty and seaside landscapes'
    },
    'nan xun gu zhen': {
        'name_en': 'Nan Xun Gu Zhen',
        'name_zh': '南浔古镇',
        'icon': 'fa-landmark',
        'description': 'Ancient water town with traditional architecture and canals'
    },
    'qian dao hu': {
        'name_en': 'Qian Dao Hu',
        'name_zh': '千岛湖',
        'icon': 'fa-water',
        'description': 'Pristine lake with countless islands and crystal clear waters'
    },
    'animals': {
        'name_en': 'Animals',
        'name_zh': '动物',
        'icon': 'fa-paw',
        'description': 'Captured moments of wildlife and nature'
    },
    'ao men': {
        'name_en': 'Ao Men',
        'name_zh': '澳门',
        'icon': 'fa-dice',
        'description': 'Vibrant cityscape and cultural experiences'
    },
    'zhu hai': {
        'name_en': 'Zhu Hai',
        'name_zh': '珠海',
        'icon': 'fa-umbrella-beach',
        'description': 'Coastal charm and seaside moments'
    },
    'on road': {
        'name_en': 'On Road',
        'name_zh': '在路上',
        'icon': 'fa-road',
        'description': 'Journey moments captured during travels'
    },
    'others': {
        'name_en': 'Others',
        'name_zh': '其他',
        'icon': 'fa-camera-retro',
        'description': 'Various captured moments and memories'
    }
}

def get_photos(category_dir):
    """Get all photo files in a category directory"""
    photos = []
    for filename in sorted(os.listdir(category_dir)):
        if filename.lower().endswith(('.jpg', '.jpeg', '.png')):
            photos.append(filename)
    return photos

def generate_page(category_dir, info, output_dir='photos'):
    """Generate HTML page for a photo category"""
    photos = get_photos(category_dir)
    category_name = os.path.basename(category_dir)
    category_slug = category_name.replace(' ', '-')

    # Build photo HTML
    photos_html = ''
    for photo in photos:
        photo_path = f"../{category_name}/{photo}"
        photos_html += f'''                        <article class="thumb">
                            <a href="{photo_path}" class="image"><img src="{photo_path}" alt="" loading="lazy" /></a>
                        </article>
'''

    html = f'''<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{info['name_en']} ({info['name_zh']}) | Photography</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css">
    <link rel="stylesheet" href="gallery.css">
    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Source+Sans+Pro:300,400">
</head>
<body>
    <!-- Header -->
    <header>
        <h1><a href="../photo-gallery.html"><i class="fas fa-arrow-left"></i> {info['name_en']}</a></h1>
        <a href="../" class="back-link"><i class="fas fa-home"></i> Home</a>
    </header>

    <!-- Main Gallery -->
    <div id="main">
{photos_html}
    </div>

    <!-- Lightbox -->
    <div id="lightbox">
        <span class="close">&times;</span>
        <span class="nav prev"><i class="fas fa-chevron-left"></i></span>
        <span class="nav next"><i class="fas fa-chevron-right"></i></span>
        <img src="" alt="">
        <div class="counter"></div>
    </div>

    <script>
        // Lightbox functionality
        const lightbox = document.getElementById('lightbox');
        const lightboxImg = lightbox.querySelector('img');
        const closeBtn = lightbox.querySelector('.close');
        const prevBtn = lightbox.querySelector('.prev');
        const nextBtn = lightbox.querySelector('.next');
        const counter = lightbox.querySelector('.counter');

        let currentIndex = 0;
        const photos = Array.from(document.querySelectorAll('.thumb .image')).map(a => a.getAttribute('href'));

        // Open lightbox
        document.querySelectorAll('.thumb .image').forEach((link, index) => {{
            link.addEventListener('click', (e) => {{
                e.preventDefault();
                currentIndex = index;
                showPhoto(index);
            }});
        }});

        // Close lightbox
        closeBtn.addEventListener('click', closeLightbox);
        lightbox.addEventListener('click', (e) => {{
            if (e.target === lightbox) closeLightbox();
        }});

        // Navigation
        prevBtn.addEventListener('click', (e) => {{
            e.stopPropagation();
            currentIndex = (currentIndex - 1 + photos.length) % photos.length;
            showPhoto(currentIndex);
        }});

        nextBtn.addEventListener('click', (e) => {{
            e.stopPropagation();
            currentIndex = (currentIndex + 1) % photos.length;
            showPhoto(currentIndex);
        }});

        // Keyboard navigation
        document.addEventListener('keydown', (e) => {{
            if (!lightbox.classList.contains('active')) return;
            if (e.key === 'Escape') closeLightbox();
            if (e.key === 'ArrowLeft') {{
                currentIndex = (currentIndex - 1 + photos.length) % photos.length;
                showPhoto(currentIndex);
            }}
            if (e.key === 'ArrowRight') {{
                currentIndex = (currentIndex + 1) % photos.length;
                showPhoto(currentIndex);
            }}
        }});

        function showPhoto(index) {{
            lightboxImg.src = photos[index];
            counter.textContent = (index + 1) + ' / ' + photos.length;
        }}

        function closeLightbox() {{
            lightbox.classList.remove('active');
        }}
    </script>
</body>
</html>
'''

    # Write file
    filename = f"{output_dir}/{category_slug}.html"
    with open(filename, 'w') as f:
        f.write(html)

    print(f"  Generated: {filename} ({len(photos)} photos)")

def main():
    print("=== Generating Photo Category Pages ===")
    print()

    total_photos = 0
    base_dir = os.path.join(os.getcwd(), 'photos')

    for category_dir, info in categories.items():
        full_path = os.path.join(base_dir, category_dir)
        if os.path.isdir(full_path):
            generate_page(full_path, info, output_dir='photos')
            photos = get_photos(full_path)
            total_photos += len(photos)

    print()
    print(f"=== Complete ===")
    print(f"Total categories: {len(categories)}")
    print(f"Total photos: {total_photos}")

if __name__ == "__main__":
    main()
