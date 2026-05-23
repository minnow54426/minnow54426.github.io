/*
	Simple Homepage JavaScript
	Loads photo gallery preview
*/

(function($) {
	// Photo Categories with URL-encoded paths
	// Using renamed photos with EXIF-based y-m-d-001.jpg format
	const photoCategories = [
		{ src: 'photos/shang%20hai/2024-08-17-001.jpg', category: 'Shanghai' },
		{ src: 'photos/shang%20hai/2026-03-08-067.jpg', category: 'Shanghai' },
		{ src: 'photos/zhang%20jia%20jie/2025-10-26-009.jpg', category: 'Zhangjiajie' },
		{ src: 'photos/qing%20dao/2026-01-02-014.jpg', category: 'Qingdao' },
		{ src: 'photos/qing%20dao/2026-01-02-015.jpg', category: 'Qingdao' },
		{ src: 'photos/qian%20dao%20hu/2024-11-10-010.jpg', category: 'Qian Dao Hu' },
		{ src: 'photos/nan%20xun%20gu%20zhen/2024-12-28-003.jpg', category: 'Nanxun Ancient Town' },
		{ src: 'photos/jiu%20zhai%20gou/2024-09-21-014.jpg', category: 'Jiuzhaigou' },
		{ src: 'photos/jiu%20zhai%20gou/2024-09-21-019.jpg', category: 'Jiuzhaigou' },
		{ src: 'photos/hu%20pao%20gong%20yuan/2024-09-01-014.jpg', category: 'Hu Pao Park' },
		{ src: 'photos/ao%20men/2025-10-09-002.jpg', category: 'Ao Men' },
		{ src: 'photos/animals/2025-10-24-023.jpg', category: 'Animals' }
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
