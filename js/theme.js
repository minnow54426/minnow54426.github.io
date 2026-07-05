/* theme.js — scroll reveal + navigation chrome for the abetkaua theme.
 * Vanilla, dependency-free, respects prefers-reduced-motion. */
(function () {
  'use strict';

  var reduce = window.matchMedia &&
    window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  function initReveal() {
    var els = document.querySelectorAll('.reveal');
    if (reduce || !('IntersectionObserver' in window)) {
      els.forEach(function (el) { el.classList.add('is-in'); });
      return;
    }
    var io = new IntersectionObserver(function (entries) {
      entries.forEach(function (entry) {
        if (entry.isIntersecting) {
          entry.target.classList.add('is-in');
          io.unobserve(entry.target);
        }
      });
    }, { threshold: 0.16, rootMargin: '0px 0px -8% 0px' });
    els.forEach(function (el) { io.observe(el); });
  }

  // Mark the current section in the topbar nav based on scroll position.
  function initNavSpy() {
    var links = document.querySelectorAll('.topbar__nav a[href^="#"]');
    if (!links.length || !('IntersectionObserver' in window)) return;
    var map = {};
    links.forEach(function (l) {
      var id = l.getAttribute('href').slice(1);
      var sec = document.getElementById(id);
      if (sec) map[id] = l;
    });
    var io = new IntersectionObserver(function (entries) {
      entries.forEach(function (entry) {
        var l = map[entry.target.id];
        if (!l) return;
        if (entry.isIntersecting) {
          links.forEach(function (x) { x.style.color = ''; });
          l.style.color = 'var(--accent)';
        }
      });
    }, { threshold: 0.4 });
    Object.keys(map).forEach(function (id) {
      io.observe(document.getElementById(id));
    });
  }

  // Lazy video: play only when in view, paused when leaving; skip if reduced motion.
  function initLazyVideo() {
    var vids = document.querySelectorAll('video[data-autoplay]');
    if (!vids.length) return;
    if (reduce || !('IntersectionObserver' in window)) {
      vids.forEach(function (v) {
        if (!reduce) v.play().catch(function () {});
      });
      return;
    }
    var io = new IntersectionObserver(function (entries) {
      entries.forEach(function (entry) {
        var v = entry.target;
        if (entry.isIntersecting) v.play().catch(function () {});
        else v.pause();
      });
    }, { threshold: 0.35 });
    vids.forEach(function (v) { io.observe(v); });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', function () {
      initReveal();
      initNavSpy();
      initLazyVideo();
    });
  } else {
    initReveal();
    initNavSpy();
    initLazyVideo();
  }
})();
