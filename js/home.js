/*
	Simple Homepage JavaScript
	Loads photo gallery preview
*/

(function($) {
	// Photo Categories with URL-encoded paths
	// Using renamed photos with y-m-d-001.jpg format
	const photoCategories = [
		{ src: 'photos/jiu%20zhai%20gou/2025-02-07-014.jpg', category: 'Jiuzhaigou' },
		{ src: 'photos/zhang%20jia%20jie/2025-02-07-006.jpg', category: 'Zhangjiajie' },
		{ src: 'photos/qian%20dao%20hu/2025-02-07-001.jpg', category: 'Qian Dao Hu' },
		{ src: 'photos/nan%20xun%20gu%20zhen/2025-02-07-003.jpg', category: 'Nanxun Ancient Town' },
		{ src: 'photos/animals/2025-02-07-003.jpg', category: 'Animals' },
		{ src: 'photos/shang%20hai/2025-02-07-047.jpg', category: 'Shanghai' },
		{ src: 'photos/ao%20men/2025-02-07-001.jpg', category: 'AoMen' },
		{ src: 'photos/gong%20qing%20sen%20lin%20gong%20yuan/2025-02-07-005.jpg', category: 'GQSLGY' }
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
