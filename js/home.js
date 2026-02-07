/**
 * Homepage Custom JavaScript
 * Handles dynamic content loading for photo gallery, video carousel, and polynomial plotter
 */

(function($) {
    'use strict';

    // ============================
    // Photo Gallery System
    // ============================

    /**
     * Photo categories configuration
     * Mapped to actual photo directories from the portfolio
     */
    const photoCategories = [
        {
            name: 'Jiu Zhai Gou',
            photos: [
                { src: 'photos/jiu zhai gou/DSC02454--export-by-date.JPG', caption: 'Colorful Lakes' },
                { src: 'photos/jiu zhai gou/DSC02456--export-by-date.JPG', caption: 'Waterfall Valley' }
            ]
        },
        {
            name: 'Zhang Jia Jie',
            photos: [
                { src: 'photos/zhang jia jie/DSC03250--export-by-date.JPG', caption: 'Sandstone Peaks' },
                { src: 'photos/zhang jia jie/DSC03253--export-by-date.JPG', caption: 'Mist Forest' }
            ]
        },
        {
            name: 'Qian Dao Hu',
            photos: [
                { src: 'photos/qian dao hu/DSC04956--export-by-date.JPG', caption: 'Thousand Island Lake' },
                { src: 'photos/qian dao hu/DSC04971--export-by-date.JPG', caption: 'Peaceful Waters' }
            ]
        },
        {
            name: 'Nan Xun Gu Zhen',
            photos: [
                { src: 'photos/nan xun gu zhen/DSC00792--export-by-date.JPG', caption: 'Ancient Water Town' },
                { src: 'photos/nan xun gu zhen/DSC00843--export-by-date.JPG', caption: 'Traditional Architecture' }
            ]
        },
        {
            name: 'Animals',
            photos: [
                { src: 'photos/animals/1--export-by-date (1).jpg', caption: 'Wildlife' },
                { src: 'photos/animals/10--export-by-date (1).jpg', caption: 'Nature Friends' }
            ]
        },
        {
            name: 'Shang Hai',
            photos: [
                { src: 'photos/shang hai/DSC01309--export-by-date.JPG', caption: 'City Skyline' },
                { src: 'photos/shang hai/DSC01311--export-by-date.JPG', caption: 'Urban Nights' }
            ]
        },
        {
            name: 'Guitar',
            photos: [
                { src: 'photos/guitar/DSC01118--export-by-date.JPG', caption: 'Acoustic Sessions' },
                { src: 'photos/guitar/DSC01127--export-by-date.JPG', caption: 'Musical Moments' }
            ]
        },
        {
            name: 'Zhu Jia Jiao Gu Zhen',
            photos: [
                { src: 'photos/zhu jia jiao gu zhen/1--export-by-date.jpg', caption: 'Historic Canals' },
                { src: 'photos/zhu jia jiao gu zhen/2--export-by-date (2).jpg', caption: 'Riverside Life' }
            ]
        }
    ];

    /**
     * Determines current season based on month
     * @returns {string} - 'spring', 'summer', 'fall', or 'winter'
     */
    function getCurrentSeason() {
        const month = new Date().getMonth(); // 0-11

        if (month >= 2 && month <= 4) return 'spring';
        if (month >= 5 && month <= 7) return 'summer';
        if (month >= 8 && month <= 10) return 'fall';
        return 'winter';
    }

    /**
     * Injects photos into the photo gallery section
     * Selects 6-8 photos from categories and builds masonry grid
     */
    function injectPhotos() {
        const gallery = $('#photoGallery');
        if (!gallery.length) return;

        const season = getCurrentSeason();
        let html = '';

        // Select photos (simplified - will be enhanced in Task 4)
        const allPhotos = photoCategories.reduce((acc, cat) => {
            return acc.concat(cat.photos.map(p => ({
                ...p,
                category: cat.name
            })));
        }, []);

        // Take first 6-8 photos
        const selectedPhotos = allPhotos.slice(0, 7);

        // Define span classes for masonry layout
        const spanClasses = ['span-3', 'span-1-5', 'span-2', 'span-2-5', 'span-3', 'span-1-5', 'span-2'];

        selectedPhotos.forEach((photo, index) => {
            const spanClass = spanClasses[index % spanClasses.length];
            html += `
                <div class="gallery-item ${spanClass}">
                    <a href="photo-gallery.html" class="image fit">
                        <img src="${photo.src}" alt="${photo.caption}" loading="lazy" />
                        <div class="overlay">
                            <span class="caption">${photo.caption}</span>
                            <span class="category">${photo.category}</span>
                        </div>
                    </a>
                </div>
            `;
        });

        gallery.html(html);
    }

    // ============================
    // Video Carousel System
    // ============================

    /**
     * Paint videos configuration
     * TODO: Configure with actual video paths in Task 5
     */
    const paintVideos = [
        {
            src: 'paint/water_color/christmas_snowman.mp4',
            thumbnail: 'paint/water_color/thumbnails/christmas_snowman.jpg',
            title: 'Christmas Snowman',
            duration: '2:15'
        },
        {
            src: 'paint/water_color/mountain.mp4',
            thumbnail: 'paint/water_color/thumbnails/mountain.jpg',
            title: 'Mountain Landscape',
            duration: '2:30'
        },
        {
            src: 'paint/water_color/flower.mp4',
            thumbnail: 'paint/water_color/thumbnails/flower.jpg',
            title: 'Floral Design',
            duration: '1:45'
        },
        {
            src: 'paint/water_color/swan.mp4',
            thumbnail: 'paint/water_color/thumbnails/swan.jpg',
            title: 'Swan on Water',
            duration: '2:00'
        }
    ];

    /**
     * Injects videos into the video carousel section
     * Creates video elements with hover-to-play behavior
     */
    function injectVideos() {
        const carousel = $('#videoCarousel');
        if (!carousel.length) return;

        let html = '<div class="video-grid">';

        paintVideos.forEach(video => {
            html += `
                <div class="video-item">
                    <a href="paint.html" class="video-link">
                        <video
                            class="video-preview"
                            src="${video.src}"
                            muted
                            loop
                            playsinline
                            preload="metadata"
                            poster="${video.thumbnail}">
                            Your browser does not support the video tag.
                        </video>
                        <div class="video-info">
                            <span class="video-title">${video.title}</span>
                            <span class="video-duration">${video.duration}</span>
                        </div>
                        <div class="arrow-button">→</div>
                    </a>
                </div>
            `;
        });

        html += '</div>';
        carousel.html(html);

        // Add hover behavior after DOM insertion
        $('.video-preview').on('mouseenter', function() {
            this.play();
        }).on('mouseleave', function() {
            this.pause();
            this.currentTime = 0;
        });
    }

    // ============================
    // Mini Polynomial Plotter
    // ============================

    /**
     * Initializes the mini polynomial plotter canvas
     * Draws axes and default polynomial
     */
    function initMiniPlotter() {
        const canvas = document.getElementById('polyCanvas');
        if (!canvas) return;

        const ctx = canvas.getContext('2d');
        const width = canvas.width;
        const height = canvas.height;

        // Clear canvas
        ctx.clearRect(0, 0, width, height);

        // Draw axes
        drawAxes(ctx, width, height);

        // Draw default polynomial (x-1)^2 = x^2 - 2x + 1
        const defaultCoeffs = [1, -2, 1];
        drawPolynomial(ctx, width, height, defaultCoeffs);
    }

    /**
     * Draws coordinate axes on the canvas
     * @param {CanvasRenderingContext2D} ctx - Canvas context
     * @param {number} width - Canvas width
     * @param {number} height - Canvas height
     */
    function drawAxes(ctx, width, height) {
        const centerX = width / 2;
        const centerY = height / 2;

        ctx.strokeStyle = '#ccc';
        ctx.lineWidth = 1;

        // X-axis
        ctx.beginPath();
        ctx.moveTo(0, centerY);
        ctx.lineTo(width, centerY);
        ctx.stroke();

        // Y-axis
        ctx.beginPath();
        ctx.moveTo(centerX, 0);
        ctx.lineTo(centerX, height);
        ctx.stroke();
    }

    /**
     * Draws a polynomial curve on the canvas
     * @param {CanvasRenderingContext2D} ctx - Canvas context
     * @param {number} width - Canvas width
     * @param {number} height - Canvas height
     * @param {Array<number>} coeffs - Polynomial coefficients [c0, c1, c2, ...]
     */
    function drawPolynomial(ctx, width, height, coeffs) {
        const centerX = width / 2;
        const centerY = height / 2;
        const scale = 60; // Pixels per unit

        // Evaluate polynomial at each x pixel
        const points = [];
        for (let px = 0; px < width; px++) {
            // Convert pixel x to coordinate x
            const x = (px - centerX) / scale;

            // Evaluate polynomial: sum of coeff[i] * x^i
            let y = 0;
            for (let i = 0; i < coeffs.length; i++) {
                y += coeffs[i] * Math.pow(x, i);
            }

            // Convert coordinate y to pixel y (flip y-axis)
            const py = centerY - (y * scale);
            points.push({ x: px, y: py });
        }

        // Draw curve
        ctx.strokeStyle = '#3498db';
        ctx.lineWidth = 3;
        ctx.beginPath();
        ctx.moveTo(points[0].x, points[0].y);
        for (let i = 1; i < points.length; i++) {
            ctx.lineTo(points[i].x, points[i].y);
        }
        ctx.stroke();

        // Draw points on curve (every 50 pixels)
        ctx.fillStyle = '#e74c3c';
        for (let i = 0; i < points.length; i += 50) {
            ctx.beginPath();
            ctx.arc(points[i].x, points[i].y, 4, 0, 2 * Math.PI);
            ctx.fill();
        }
    }

    /**
     * Sets up click handlers for polynomial preset buttons
     * Clears canvas and redraws with selected polynomial
     */
    function setupPlotterButtons() {
        const canvas = document.getElementById('polyCanvas');
        if (!canvas) return;

        const ctx = canvas.getContext('2d');
        const width = canvas.width;
        const height = canvas.height;

        // Simple parabola: (x-1)^2
        $('#btn-simple').on('click', function() {
            ctx.clearRect(0, 0, width, height);
            drawAxes(ctx, width, height);
            drawPolynomial(ctx, width, height, [1, -2, 1]);
        });

        // Complex cubic: 2x^3 - 3x^2 + 1
        $('#btn-complex').on('click', function() {
            ctx.clearRect(0, 0, width, height);
            drawAxes(ctx, width, height);
            drawPolynomial(ctx, width, height, [1, 0, -3, 2]);
        });

        // ZK demo: hyperbola x^2 - 1
        $('#btn-zk-demo').on('click', function() {
            ctx.clearRect(0, 0, width, height);
            drawAxes(ctx, width, height);
            drawPolynomial(ctx, width, height, [-1, 0, 1]);
        });
    }

    // ============================
    // Initialization
    // ============================

    $(document).ready(function() {
        // Initialize all dynamic content
        injectPhotos();
        injectVideos();
        initMiniPlotter();
        setupPlotterButtons();
    });

})(jQuery);
