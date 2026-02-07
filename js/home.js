/*
	Simple Homepage JavaScript
	Loads photo gallery preview
*/

(function($) {
	// Photo Categories with URL-encoded paths
	const photoCategories = [
		{ src: 'photos/jiu%20zhai%20gou/DSC02572--export-by-date.JPG', category: 'Jiuzhaigou' },
		{ src: 'photos/zhang%20jia%20jie/8--export-by-date (2).JPG', category: 'Zhangjiajie' },
		{ src: 'photos/qian%20dao%20hu/f5de7c3f951cc027f4ef8241772122f4--export-by-date.JPG', category: 'Qian Dao Hu' },
		{ src: 'photos/nan%20xun%20gu%20zhen/DSC04978--export-by-date.JPG', category: 'Nanxun Ancient Town' },
		{ src: 'photos/animals/DSC04236--export-by-date.jpg', category: 'Animals' },
		{ src: 'photos/shang%20hai/DSC02828--export-by-date.JPG', category: 'Shanghai' },
		{ src: 'photos/ao%20men/9--export-by-date.JPG', category: 'AoMen' },
		{ src: 'photos/gong%20qing%20sen%20lin%20gong%20yuan/1--export-by-date (5).JPG', category: 'GQSLGY' }
	];

	// Inject photos into gallery preview
	function injectPhotos() {
		const gallery = $('#photoGallery');
		if (!gallery.length) return;

		let html = '';
		photoCategories.forEach(photo => {
			html += `<img src="${photo.src}" alt="${photo.category}">`;
		});

		gallery.html(html);
	}

	// Initialize on page load
	$(document).ready(function() {
		injectPhotos();
	});

})(jQuery);
