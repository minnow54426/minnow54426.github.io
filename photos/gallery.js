/*
Gallery Initialization for HTML5 UP Multiverse Template
Implements background-image technique and Poptrox modal popup
*/

(function($) {

	'use strict';

	// Wait for DOM to be ready
	$(function() {
		var $main = $('#main');
		var $body = $('body');

		// Thumbs: Set background images (template technique)
		$main.children('.thumb').each(function() {
			var $this = $(this);
			var $image = $this.find('.image');
			var $image_img = $image.children('img');
			var x;

			// No image? Bail.
			if ($image.length == 0)
				return;

			// Set background image from img src
			$image.css('background-image', 'url(' + $image_img.attr('src') + ')');

			// Set background position if data-position attribute exists
			if (x = $image_img.data('position')) {
				$image.css('background-position', x);
			}

			// Hide original img (image now shown as CSS background)
			$image_img.hide();
		});

		// Poptrox: Modal popup configuration (images only)
		$main.poptrox({
			baseZIndex: 20000,
			caption: function($a) {
				return ''; // No captions - images only
			},
			fadeSpeed: 300,
			onPopupClose: function() {
				$body.removeClass('modal-active');
			},
			onPopupOpen: function() {
				$body.addClass('modal-active');
			},
			overlayOpacity: 0,
			popupCloserText: '',
			popupHeight: 150,
			popupLoaderText: '',
			popupSpeed: 300,
			popupWidth: 150,
			selector: '.thumb > a.image',
			usePopupCaption: false,    // No captions
			usePopupCloser: true,      // Show close button
			usePopupDefaultStyling: false,
			usePopupForceClose: true,  // Click outside to close
			usePopupLoader: true,      // Show loading spinner
			usePopupNav: true,         // Next/Prev navigation arrows
			windowMargin: 50           // Margin around modal
		});

		// Adjust window margin for mobile
		if (breakpoints) {
			breakpoints.on('<=xsmall', function() {
				if ($main[0]._poptrox) {
					$main[0]._poptrox.windowMargin = 0;
				}
			});

			breakpoints.on('>xsmall', function() {
				if ($main[0]._poptrox) {
					$main[0]._poptrox.windowMargin = 50;
				}
			});
		}
	});

})(jQuery);
