/*
	Simple Homepage JavaScript
	Loads photo gallery preview
*/

(function($) {
	// Photo Categories with URL-encoded paths
	const photoCategories = [
		{ src: 'photos/jiu%20zhai%20gou/DSC02454--export-by-date.JPG', category: 'Jiuzhaigou' },
		{ src: 'photos/zhang%20jia%20jie/DSC03250--export-by-date.JPG', category: 'Zhangjiajie' },
		{ src: 'photos/qian%20dao%20hu/DSC04956--export-by-date.JPG', category: 'Qian Dao Hu' },
		{ src: 'photos/nan%20xun%20gu%20zhen/DSC00792--export-by-date.JPG', category: 'Nanxun Ancient Town' },
		{ src: 'photos/animals/1--export-by-date%20(1).jpg', category: 'Animals' },
		{ src: 'photos/shang%20hai/DSC01309--export-by-date.JPG', category: 'Shanghai' }
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
