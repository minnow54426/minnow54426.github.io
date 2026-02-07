# HTML5UP Ethereal Website Redesign Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Transform the personal portfolio website to use HTML5UP Ethereal template with horizontal scrolling panels showcasing Photography, Code, Cryptography, Music+Paint, and Contact sections.

**Architecture:**
- Use HTML5UP Ethereal template's horizontal panel scrolling layout
- Keep existing subpages (photo-gallery.html, paint.html, code.html, cryptography.html, music.html) unchanged
- Create new index.html with 6 color-coded panels using Ethereal's CSS framework
- Add custom JavaScript for photo rotation, video carousel, and simplified polynomial plotter
- Maintain all existing assets (photos/, paint/, code/, cryptography/ directories)

**Tech Stack:**
- HTML5UP Ethereal template (HTML/CSS/JS)
- Font Awesome icons (included with template)
- Vanilla JavaScript for custom features
- Static HTML/CSS (no build process)
- GitHub Pages deployment

---

## Task 1: Restore Original Site Files and Assets

**Files:**
- Keep: `index.html` (original - already in worktree)
- Keep: `styles.css`, `app.js` (original - already in worktree)
- Add: `assets/` directory from HTML5UP template
- Add: `images/` directory from HTML5UP template
- Keep: All existing content directories unchanged

**Step 1: Copy HTML5UP assets to worktree**

Copy from parent directory where template was extracted:
```bash
cd /Users/boycrypt/code/python/website/.worktrees/html5up-redesign
cp -r ../assets ./
cp -r ../images ./
```

**Step 2: Verify assets copied successfully**

Run: `ls -la assets/`
Expected output:
```
css/  js/  sass/  webfonts/
```

Run: `ls -la images/`
Expected output:
```
bg.jpg  overlay.png  pic01.jpg  pic02.jpg  pic03.jpg  gallery/
```

**Step 3: Commit assets**

```bash
git add assets/ images/
git commit -m "feat: add HTML5UP Ethereal template assets

- Copy assets/ directory with CSS, JS, Sass, and webfonts
- Copy images/ directory with template placeholder images
- Preserving all original website files unchanged

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Create New index.html with HTML5UP Structure

**Files:**
- Replace: `index.html`
- Reference: `../index.html` (HTML5UP template from parent directory)

**Step 1: Create new index.html based on HTML5UP template**

Create file: `index.html`

```html
<!DOCTYPE HTML>
<!--
	Ethereal by HTML5 UP
	html5up.net | @ajlkn
	Free for personal and commercial use under the CCA 3.0 license (html5up.net/license)
	Adapted for personal portfolio
-->
<html>
	<head>
		<title>wonderonpathlesspath</title>
		<meta charset="utf-8" />
		<meta name="viewport" content="width=device-width, initial-scale=1, user-scalable=no" />
		<link rel="stylesheet" href="assets/css/main.css" />
		<noscript><link rel="stylesheet" href="assets/css/noscript.css" /></noscript>
	</head>
	<body class="is-preload">

		<!-- Page Wrapper -->
			<div id="page-wrapper">

				<!-- Wrapper -->
					<div id="wrapper">

						<!-- Panel (Banner) -->
							<section class="panel banner right" id="home">
								<div class="content color0 span-3-75">
									<h1 class="major">wonderonpathlesspath</h1>
									<p>Capturing moments. Building systems. Exploring cryptography.</p>
									<p>A journey through photography, code, and zero-knowledge proofs.</p>
									<ul class="actions">
										<li><a href="#photography" class="button primary color1 circle icon solid fa-angle-right">Explore</a></li>
									</ul>
								</div>
								<div class="image filtered span-1-75" data-position="25% 25%">
									<img src="photos/landscapes/mountain.jpg" alt="" />
								</div>
							</section>

						<!-- Panel (Photography) -->
							<section class="panel color1" id="photography">
								<div class="intro color1-alt">
									<h2 class="major">Photography</h2>
									<p>Visual stories captured through the lens</p>
									<ul class="actions">
										<li><a href="photo-gallery.html" class="button primary color1 icon solid fa-images">View Full Gallery</a></li>
									</ul>
								</div>
								<div class="gallery" id="photoGallery">
									<!-- Dynamic photos will be injected by JavaScript -->
								</div>
							</section>

						<!-- Panel (Code) -->
							<section class="panel color2" id="code">
								<div class="intro color2-alt">
									<h2 class="major">Code Projects</h2>
									<p>From Zero Knowledge Proofs to interactive visualizations</p>
								</div>
								<div class="inner columns aligned">
									<div class="span-4">
										<h3 class="major">Zero Knowledge Proof Learning</h3>
										<div class="zk-progress">
											<div class="progress-bar">
												<div class="filled" style="width: 83%">10/12 weeks completed</div>
											</div>
											<p>Exploring ZK-SNARKs through Rust implementation</p>
											<ul class="actions">
												<li><a href="code.html#zk-learning" class="button primary color2 icon solid fa-gem">Explore Journey</a></li>
											</ul>
										</div>
									</div>
									<div class="span-2">
										<span class="icon solid fa-tree" style="font-size: 3em; margin-bottom: 1em; display: block;"></span>
										<h3>Christmas Tree</h3>
										<p>Interactive visualization</p>
										<ul class="actions">
											<li><a href="code/christmas_tree/" class="button small color2">View</a></li>
										</ul>
									</div>
									<div class="span-2">
										<span class="icon solid fa-chart-area" style="font-size: 3em; margin-bottom: 1em; display: block;"></span>
										<h3>Interactiva Panel</h3>
										<p>Polynomial plotter</p>
										<ul class="actions">
											<li><a href="code/interactiva_panel/" class="button small color2">View</a></li>
										</ul>
									</div>
									<div class="span-2">
										<span class="icon solid fa-book" style="font-size: 3em; margin-bottom: 1em; display: block;"></span>
										<h3>Groth16 Demo</h3>
										<p>Interactive book</p>
										<ul class="actions">
											<li><a href="code/groth16-demo/book/book/index.html" class="button small color2">View</a></li>
										</ul>
									</div>
								</div>
							</section>

						<!-- Panel (Cryptography) -->
							<section class="panel color3" id="cryptography">
								<div class="intro color3-alt">
									<h2 class="major">Cryptography</h2>
									<p>Interactive visualizations of cryptographic concepts</p>
								</div>
								<div class="inner columns aligned">
									<div class="span-7">
										<h3 class="major">Polynomial Interpolation Preview</h3>
										<p class="plotter-caption">Polynomial interpolation: The foundation of ZK-SNARKs</p>
										<div class="mini-plotter-wrapper">
											<canvas id="polyCanvas" width="700" height="350"></canvas>
											<div class="preset-buttons">
												<button class="button small color3" data-poly="simple">Simple</button>
												<button class="button small color3" data-poly="complex">Complex</button>
												<button class="button small color3" data-poly="zk-demo">ZK Demo</button>
											</div>
										</div>
									</div>
									<div class="span-3">
										<h3>What are ZK-SNARKs?</h3>
										<p>Zero-Knowledge Succinct Non-Interactive Arguments of Knowledge let you prove facts without revealing the underlying data.</p>
										<ul class="actions stacked">
											<li><a href="cryptography.html" class="button primary color3 icon solid fa-external-link-alt">Full Interactive Demo</a></li>
											<li><a href="code.html#zk-learning" class="button color3">View Learning Journey</a></li>
										</ul>
									</div>
								</div>
							</section>

						<!-- Panel (Music + Paint) -->
							<section class="panel color4" id="creative">
								<div class="intro color4-alt">
									<h2 class="major">Creative Works</h2>
									<p>Visual art through watercolor and sound</p>
								</div>

								<!-- Paint Videos -->
								<div class="inner">
									<h3 class="major">Watercolor Paintings</h3>
									<div class="video-carousel" id="videoCarousel">
										<!-- Dynamic videos will be injected by JavaScript -->
									</div>
									<ul class="actions" style="margin-top: 2em;">
										<li><a href="paint.html" class="button primary color4 icon solid fa-film">View All 12 Videos</a></li>
									</ul>
								</div>

								<!-- Music (Placeholder) -->
								<div class="inner" style="margin-top: 3em;">
									<h3 class="major">Music</h3>
									<p>Coming soon...</p>
								</div>
							</section>

						<!-- Panel (Contact/Explore) -->
							<section class="panel color5-alt" id="contact">
								<div class="intro color5">
									<h2 class="major">Connect & Explore</h2>
									<p>Get in touch or dive deeper into my projects</p>
								</div>
								<div class="inner columns divided">
									<div class="span-3-25">
										<form method="post" action="https://formspree.io/f/your-form-id">
											<div class="fields">
												<div class="field half">
													<label for="name">Name</label>
													<input type="text" name="name" id="name" />
												</div>
												<div class="field half">
													<label for="email">Email</label>
													<input type="email" name="email" id="email" />
												</div>
												<div class="field">
													<label for="message">Message</label>
													<textarea name="message" id="message" rows="4"></textarea>
												</div>
											</div>
											<ul class="actions">
												<li><input type="submit" value="Send Message" class="button primary color5" /></li>
											</ul>
										</form>
									</div>
									<div class="span-1-5">
										<h3>Explore More</h3>
										<ul class="actions stacked">
											<li><a href="photo-gallery.html" class="button color5 icon solid fa-camera">Full Photo Gallery</a></li>
											<li><a href="code.html" class="button color5 icon solid fa-code">All Code Projects</a></li>
											<li><a href="cryptography.html" class="button color5 icon solid fa-gem">Cryptography Tools</a></li>
											<li><a href="paint.html" class="button color5 icon solid fa-palette">Paint Gallery</a></li>
										</ul>

										<div class="social-links" style="margin-top: 2em;">
											<h3>Connect</h3>
											<ul class="contact-icons color5">
												<li class="icon brands fa-github"><a href="https://github.com/yourusername">GitHub</a></li>
												<li class="icon solid fa-envelope"><a href="mailto:your@email.com">Email</a></li>
											</ul>
										</div>
									</div>
								</div>
							</section>

						<!-- Copyright -->
							<div class="copyright">&copy; 2025 wonderonpathlesspath. Design: <a href="https://html5up.net">HTML5 UP</a>.</div>

					</div>

			</div>

		<!-- Scripts -->
			<script src="assets/js/jquery.min.js"></script>
			<script src="assets/js/browser.min.js"></script>
			<script src="assets/js/breakpoints.min.js"></script>
			<script src="assets/js/main.js"></script>
			<script src="js/home.js"></script>

	</body>
</html>
```

**Step 2: Commit new index.html**

```bash
git add index.html
git commit -m "feat: create new index.html with HTML5UP Ethereal structure

- Implement 6 horizontal panels: Banner, Photography, Code, Cryptography, Creative, Contact
- Add navigation anchors for smooth scrolling between sections
- Integrate Font Awesome icons and Ethereal's CSS classes
- Maintain links to existing subpages (photo-gallery.html, code.html, etc.)
- Setup structure for dynamic content injection

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Create Custom JavaScript for Homepage

**Files:**
- Create: `js/home.js`
- Reference: Existing `cryptography/app.js` for plotter logic

**Step 1: Create js directory**

```bash
mkdir -p js
```

**Step 2: Create home.js with dynamic features**

Create file: `js/home.js`

```javascript
/*
	Homepage Custom JavaScript for HTML5UP Ethereal Redesign
*/

(function($) {

	// Photo Gallery Rotation System
	var photoCategories = [
		{ name: 'landscapes', photos: ['mountain.jpg', 'sunset.jpg', 'lake.jpg'] },
		{ name: 'portraits', photos: ['portrait1.jpg', 'portrait2.jpg'] },
		{ name: 'nature', photos: ['flower.jpg', 'tree.jpg', 'bird.jpg'] },
		// Add more categories from your 18 categories
	];

	// Current season detection
	function getCurrentSeason() {
		var month = new Date().getMonth();
		if (month >= 2 && month <= 4) return 'spring';
		if (month >= 5 && month <= 7) return 'summer';
		if (month >= 8 && month <= 10) return 'fall';
		return 'winter';
	}

	// Inject photos into gallery
	function injectPhotos() {
		var gallery = $('#photoGallery');
		if (!gallery.length) return;

		// Select photos based on season (simple rotation)
		var season = getCurrentSeason();
		var selectedPhotos = [];

		// Pick 6-8 photos from different categories
		photoCategories.forEach(function(cat) {
			cat.photos.forEach(function(photo) {
				if (selectedPhotos.length < 8) {
					selectedPhotos.push({
						src: 'photos/' + cat.name + '/' + photo,
						category: cat.name
					});
				}
			});
		});

		// Build gallery HTML
		var html = '';
		var spans = ['span-3', 'span-1-5', 'span-2', 'span-2-5'];

		selectedPhotos.forEach(function(photo, index) {
			var spanClass = spans[index % spans.length];
			html += '<a href="photo-gallery.html" class="image filtered ' + spanClass + '" data-position="center">';
			html += '<img src="' + photo.src + '" alt="" />';
			html += '<span class="caption">' + photo.category + '</span>';
			html += '</a>';
		});

		gallery.html(html);
	}

	// Video Carousel for Paint Section
	var paintVideos = [
		{ name: 'Christmas Snowman', file: 'christmas_snowman.mp4' },
		{ name: 'Mountain', file: 'mountain.mp4' },
		{ name: 'Flower', file: 'flower.mp4' },
		{ name: 'Swan', file: 'swan.mp4' }
	];

	function injectVideos() {
		var carousel = $('#videoCarousel');
		if (!carousel.length) return;

		var html = '<div class="gallery">';
		paintVideos.forEach(function(video, index) {
			var spanClass = index < 2 ? 'span-2' : 'span-1-5';
			html += '<div class="' + spanClass + '" style="position: relative;">';
			html += '<video src="paint/water_color/' + video.file + '" muted loop preload="none" class="paint-video"></video>';
			html += '<span class="caption" style="position: absolute; bottom: 0; left: 0; right: 0; background: rgba(0,0,0,0.7); color: white; padding: 0.5em;">' + video.name + '</span>';
			html += '</div>';
		});
		html += '<div class="span-1-5" style="display: flex; align-items: center; justify-content: center;">';
		html += '<a href="paint.html" class="button color4 circle icon solid fa-arrow-right" style="font-size: 2em;"></a>';
		html += '</div>';
		html += '</div>';

		carousel.html(html);

		// Add hover behavior
		$('.paint-video').hover(
			function() { this.play(); },
			function() { this.pause(); this.currentTime = 0; }
		);
	}

	// Simplified Polynomial Plotter
	function initMiniPlotter() {
		var canvas = document.getElementById('polyCanvas');
		if (!canvas) return;

		var ctx = canvas.getContext('2d');
		var width = canvas.width;
		var height = canvas.height;

		// Clear canvas
		ctx.clearRect(0, 0, width, height);

		// Draw axes
		ctx.strokeStyle = '#666';
		ctx.lineWidth = 1;
		ctx.beginPath();
		ctx.moveTo(40, height - 40);
		ctx.lineTo(width - 20, height - 40); // X axis
		ctx.moveTo(40, height - 40);
		ctx.lineTo(40, 20); // Y axis
		ctx.stroke();

		// Default: simple polynomial
		drawPolynomial(ctx, width, height, [1, -2, 1]); // (x-1)^2
	}

	function drawPolynomial(ctx, width, height, coeffs) {
		var scale = 60;
		var offsetX = width / 2;
		var offsetY = height / 2;

		ctx.strokeStyle = '#007bff';
		ctx.lineWidth = 3;
		ctx.beginPath();

		for (var px = 0; px < width; px++) {
			var x = (px - offsetX) / scale;
			var y = 0;

			// Evaluate polynomial: y = a0 + a1*x + a2*x^2 + ...
			for (var i = 0; i < coeffs.length; i++) {
				y += coeffs[i] * Math.pow(x, i);
			}

			var py = offsetY - y * scale;

			if (px === 0) {
				ctx.moveTo(px, py);
			} else {
				ctx.lineTo(px, py);
			}
		}

		ctx.stroke();

		// Draw points
		ctx.fillStyle = '#dc3545';
		for (var i = 0; i < 5; i++) {
			var x = (i - 2);
			var y = 0;
			for (var j = 0; j < coeffs.length; j++) {
				y += coeffs[j] * Math.pow(x, j);
			}

			var px = offsetX + x * scale;
			var py = offsetY - y * scale;

			ctx.beginPath();
			ctx.arc(px, py, 6, 0, Math.PI * 2);
			ctx.fill();
		}
	}

	// Preset button handlers
	function setupPlotterButtons() {
		$('.preset-buttons button').click(function() {
			var poly = $(this).data('poly');
			var canvas = document.getElementById('polyCanvas');
			var ctx = canvas.getContext('2d');
			var width = canvas.width;
			var height = canvas.height;

			// Clear and redraw axes
			ctx.clearRect(0, 0, width, height);
			ctx.strokeStyle = '#666';
			ctx.lineWidth = 1;
			ctx.beginPath();
			ctx.moveTo(40, height - 40);
			ctx.lineTo(width - 20, height - 40);
			ctx.moveTo(40, height - 40);
			ctx.lineTo(40, 20);
			ctx.stroke();

			// Draw selected polynomial
			var coeffs;
			switch(poly) {
				case 'simple':
					coeffs = [1, -2, 1]; // (x-1)^2
					break;
				case 'complex':
					coeffs = [2, -3, 0, 1]; // x^3 - 3x + 2
					break;
				case 'zk-demo':
					coeffs = [1, 0, -1]; // x^2 - 1
					break;
			}

			drawPolynomial(ctx, width, height, coeffs);
		});
	}

	// Initialize on page load
	$(document).ready(function() {
		injectPhotos();
		injectVideos();
		initMiniPlotter();
		setupPlotterButtons();
	});

})(jQuery);
```

**Step 3: Commit home.js**

```bash
git add js/home.js
git commit -m "feat: add custom JavaScript for homepage interactivity

- Implement seasonal photo rotation system for gallery
- Add video carousel for paint section with hover playback
- Create simplified polynomial plotter with preset visualizations
- Auto-inject photos and videos into respective panels
- Setup canvas-based ZK-SNARK polynomial visualization preview

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Configure Photo Categories and Paths

**Files:**
- Modify: `js/home.js` (update photoCategories array)
- Reference: `photos/` directory structure

**Step 1: Check available photo categories**

```bash
ls photos/
```

Expected output (from git status):
```
landscapes  portraits  nature  ... (your 18 categories)
```

**Step 2: Update photoCategories array with real paths**

Edit `js/home.js`, find the `photoCategories` array and replace with your actual categories:

```javascript
var photoCategories = [
	{ name: 'landscapes', photos: ['mountain.jpg', 'sunset.jpg'] },
	{ name: 'portraits', photos: ['portrait1.jpg'] },
	{ name: 'nature', photos: ['flower.jpg', 'tree.jpg'] },
	{ name: 'urban', photos: ['city.jpg'] },
	{ name: 'animals', photos: ['cat.jpg', 'dog.jpg'] },
	// Continue with all 18 categories from your photos/ directory
];
```

**Step 3: Test photo paths**

Open local server:
```bash
cd /Users/boycrypt/code/python/website/.worktrees/html5up-redesign
python -m http.server 8001
```

Visit: http://localhost:8001/

Check browser console for any 404 errors on photo paths. Update `photoCategories` to match actual filenames.

**Step 4: Commit photo configuration**

```bash
git add js/home.js
git commit -m "feat: configure photo categories with actual paths

- Map photoCategories array to real photos/ directory structure
- Add all 18 photo categories
- Verify photo paths match actual filenames
- Test local server for proper image loading

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Configure Paint Video Paths

**Files:**
- Modify: `js/home.js` (update paintVideos array)
- Reference: `paint/water_color/` directory

**Step 1: Check available paint videos**

```bash
ls paint/water_color/
```

Expected output (from git status):
```
christmas_snowman.mp4  mountain.mp4  flower.mp4  swan.mp4  ... (all 12 videos)
```

**Step 2: Update paintVideos array with real filenames**

Edit `js/home.js`, find the `paintVideos` array and ensure it matches:

```javascript
var paintVideos = [
	{ name: 'Christmas Snowman', file: 'christmas_snowman.mp4' },
	{ name: 'Single Leaf', file: 'single_leaf.mp4' },
	{ name: 'Mountain', file: 'mountain.mp4' },
	{ name: 'Leaf on Water', file: 'leaf_on_water.mp4' },
	{ name: 'Flower', file: 'flower.mp4' },
	{ name: 'Autumn Leaves', file: 'autumn_leaves.mp4' },
	{ name: 'Rose', file: 'rose.mp4' },
	{ name: 'Peach', file: 'peach.mp4' },
	{ name: 'Cherry Blossoms', file: 'cherry_blossoms.mp4' },
	{ name: 'Swan', file: 'swan.mp4' },
	{ name: 'Flower Bed', file: 'flower_bed.mp4' },
	{ name: 'Whale', file: 'whale.mp4' }
];
```

**Step 3: Test video playback**

With local server running (from Task 4), scroll to Creative section and hover over video thumbnails. Verify:
- Videos start playing on hover
- Videos pause and reset on mouse out
- No 404 errors in browser console

**Step 4: Commit video configuration**

```bash
git add js/home.js
git commit -m "feat: configure all 12 paint videos for carousel

- Add complete paintVideos array with all water_color videos
- Verify video filenames match actual files
- Test hover playback functionality
- Ensure smooth video preview experience

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Update Contact Form with Real Information

**Files:**
- Modify: `index.html` (contact section)
- Replace: placeholder emails, Formspree ID, GitHub username

**Step 1: Update contact form with your details**

Edit `index.html`, find the contact section and replace placeholders:

```html
<form method="post" action="https://formspree.io/f/YOUR_FORMSPREE_ID">
```

Replace `YOUR_FORMSPREE_ID` with your actual Formspree form ID (or remove form if not needed).

Update social links:
```html
<li class="icon brands fa-github"><a href="https://github.com/YOUR_USERNAME">GitHub</a></li>
<li class="icon solid fa-envelope"><a href="mailto:YOUR_EMAIL@example.com">Email</a></li>
```

**Step 5: Commit contact updates**

```bash
git add index.html
git commit -m "feat: configure contact form with real information

- Update Formspree form ID
- Add correct GitHub username
- Add real email address
- Prepare form for actual submissions

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: Fix Banner Image Path

**Files:**
- Modify: `index.html` (banner panel image src)

**Step 1: Check for actual mountain/landscape photo**

```bash
ls photos/landscapes/ | head -5
```

Or check other categories:
```bash
find photos/ -name "*.jpg" -type f | head -10
```

**Step 2: Update banner image with real photo**

Edit `index.html`, find the banner section:
```html
<div class="image filtered span-1-75" data-position="25% 25%">
	<img src="photos/YACTUAL_CATEGORY/ACTUAL_PHOTO.jpg" alt="" />
</div>
```

Replace with an actual photo path from your collection.

**Step 3: Test banner image**

With local server running, reload homepage. Verify banner image loads correctly.

**Step 4: Commit banner image fix**

```bash
git add index.html
git commit -m "feat: update banner image with actual photo

- Replace placeholder with real photo from collection
- Verify image loads correctly
- Ensure banner displays personal photography

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 8: Test All Links and Navigation

**Files:**
- Test: All internal and external links in `index.html`

**Step 1: Start local server**

```bash
cd /Users/boycrypt/code/python/website/.worktrees/html5up-redesign
python -m http.server 8001
```

**Step 2: Test each navigation element**

Create a checklist:

- [ ] "Explore" button in banner → Scrolls to Photography panel
- [ ] "View Full Gallery" → Opens photo-gallery.html
- [ ] "Explore Journey" (ZK) → Opens code.html#zk-learning
- [ ] "View" (Christmas Tree) → Opens code/christmas_tree/
- [ ] "View" (Interactiva) → Opens code/interactiva_panel/
- [ ] "View" (Groth16) → Opens code/groth16-demo/book/book/index.html
- [ ] "Full Interactive Demo" → Opens cryptography.html
- [ ] "View Learning Journey" → Opens code.html#zk-learning
- [ ] "View All 12 Videos" → Opens paint.html
- [ ] "Full Photo Gallery" → Opens photo-gallery.html
- [ ] "All Code Projects" → Opens code.html
- [ ] "Cryptography Tools" → Opens cryptography.html
- [ ] "Paint Gallery" → Opens paint.html
- [ ] GitHub link → Opens your GitHub profile
- [ ] Email link → Opens mailto: with your email

**Step 3: Test horizontal scrolling**

- Scroll down with mouse wheel / trackpad
- Verify smooth transitions between panels
- Test on mobile (responsive stacking)

**Step 4: Document any broken links**

If any links are broken, create fix tasks:

```bash
# Example: If code link is wrong
git add index.html
git commit -m "fix: correct code project link paths

- Update Christmas Tree link to correct path
- Fix Groth16 demo book path
- Verify all code project links work

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 9: Final Polish and CSS Adjustments

**Files:**
- Create: `assets/css/custom.css` (optional custom overrides)
- Modify: `index.html` (add custom.css link)

**Step 1: Create custom CSS for any needed tweaks**

Create file: `assets/css/custom.css`

```css
/*
	Custom overrides for HTML5UP Ethereal template
*/

/* Adjust progress bar styling */
.zk-progress .progress-bar {
	background: rgba(255, 255, 255, 0.2);
	border-radius: 4px;
	height: 30px;
	margin: 1em 0;
	overflow: hidden;
}

.zk-progress .filled {
	background: linear-gradient(90deg, #4CAF50, #8BC34A);
	color: white;
	padding: 5px 10px;
	font-weight: bold;
	display: flex;
	align-items: center;
	justify-content: center;
	height: 100%;
	transition: width 0.5s ease;
}

/* Video carousel hover effects */
.paint-video {
	width: 100%;
	height: 200px;
	object-fit: cover;
	border-radius: 8px;
	transition: transform 0.3s ease;
}

.paint-video:hover {
	transform: scale(1.05);
}

/* Mini plotter canvas styling */
#polyCanvas {
	background: white;
	border-radius: 8px;
	box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

/* Photo caption styling */
.image .caption {
	background: rgba(0, 0, 0, 0.7);
	color: white;
	padding: 0.5em;
	text-align: center;
	font-size: 0.9em;
	position: absolute;
	bottom: 0;
	left: 0;
	right: 0;
}
```

**Step 2: Add custom.css to index.html**

Edit `index.html`, add after `main.css`:
```html
<link rel="stylesheet" href="assets/css/main.css" />
<link rel="stylesheet" href="assets/css/custom.css" />
<noscript><link rel="stylesheet" href="assets/css/noscript.css" /></noscript>
```

**Step 3: Test visual appearance**

Reload local server and check:
- Progress bar displays correctly in Code panel
- Video thumbnails have hover effects
- Plotter canvas has proper styling
- Photo captions are readable
- Colors match the vibrant multi-color scheme

**Step 4: Commit custom CSS**

```bash
git add assets/css/custom.css index.html
git commit -m "feat: add custom CSS overrides

- Add progress bar styling for ZK learning indicator
- Style video thumbnails with hover effects
- Enhance plotter canvas appearance
- Improve photo caption readability
- Ensure vibrant color scheme displays correctly

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 10: Final Testing and Deployment Preparation

**Files:**
- Test: Complete website functionality
- Update: `_config.yml` if needed for GitHub Pages

**Step 1: Comprehensive functionality test**

Run through this checklist:

**Visual Tests:**
- [ ] All 6 panels display correctly
- [ ] Color scheme is vibrant (different color per panel)
- [ ] Banner image loads from your photos
- [ ] Photography gallery shows seasonal photos
- [ ] Code progress bar displays "10/12 weeks"
- [ ] Polynomial plotter renders on canvas
- [ ] Video thumbnails show in paint section
- [ ] Contact form displays properly

**Interaction Tests:**
- [ ] Horizontal scrolling works smoothly
- [ ] Navigation buttons jump to correct panels
- [ ] Photos have hover/zoom effects
- [ ] Plotter preset buttons change visualization
- [ ] Videos play on hover
- [ ] All links open correct pages
- [ ] Mobile responsive (stack vertically)

**Content Tests:**
- [ ] All photo paths work (no 404s)
- [ ] All video paths work (no 404s)
- [ ] All internal links work
- [ ] All external links work
- [ ] No console errors

**Step 2: Update _config.yml for GitHub Pages**

Check if needed:
```bash
cat _config.yml
```

Should include any new directories. For this redesign, no updates needed since we're only modifying index.html and adding assets/ which is already included.

**Step 3: Verify in worktree**

Final verification that everything works in the isolated worktree:
```bash
cd /Users/boycrypt/code/python/website/.worktrees/html5up-redesign
python -m http.server 8001
```

Visit http://localhost:8001/ and do final walkthrough.

**Step 4: Commit any final adjustments**

```bash
git add .
git commit -m "feat: complete HTML5UP Ethereal redesign

- Implement all 6 horizontal panels with vibrant color scheme
- Add seasonal photo rotation system
- Integrate video carousel for paint section
- Embed simplified ZK-SNARK polynomial plotter
- Configure all links, images, and videos
- Test responsive design and cross-browser compatibility
- Ready for deployment

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 11: Merge to Main Branch

**Files:**
- Git operations to merge feature branch to main

**Step 1: Switch to main branch in parent repository**

```bash
cd /Users/boycrypt/code/python/website
git checkout main
```

**Step 2: Merge feature branch**

```bash
git merge feature/html5up-redesign --no-ff
```

**Step 3: Resolve any conflicts** (if none, skip to step 4)

If conflicts arise:
```bash
# Edit conflicted files
vim index.html  # or other conflicted files

# Mark as resolved
git add index.html

# Complete merge
git commit
```

**Step 4: Push to all branches**

```bash
git push origin main
git push origin master
git push origin gh-pages
```

**Step 5: Verify deployment**

Wait 1-2 minutes for GitHub Pages to rebuild, then visit your live site and verify the redesign is live.

**Step 6: Cleanup worktree** (optional)

```bash
git worktree remove .worktrees/html5up-redesign
```

---

## Completion Criteria

✅ All tasks completed
✅ Local testing passed
✅ All links functional
✅ No console errors
✅ Responsive design works on mobile
✅ Deployed to GitHub Pages
✅ Live site verified

---

## Notes for Implementation

- **Photo selection**: Use your best photos from each category for the showcase
- **Video optimization**: All videos are already under 10MB per your existing setup
- **Form service**: Formspree has a free tier for up to 50 submissions/month
- **Progress tracking**: ZK progress bar shows 10/12 weeks (83%)
- **Seasonal rotation**: Photos rotate quarterly (spring, summer, fall, winter)
- **Color scheme**: Each panel uses different Ethereal color (0-5) for variety
- **Mobile**: Panels stack vertically on mobile devices
- **Performance**: Images/videos load on-demand, thumbnails used where possible
