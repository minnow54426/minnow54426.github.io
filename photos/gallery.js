/*
Gallery Initialization for HTML5 UP Multiverse Template
Uses direct <img> display instead of background-image technique
*/

(function($) {

	'use strict';

	// Wait for DOM to be ready
	$(function() {
		var $main = $('#main');
		var $body = $('body');

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
