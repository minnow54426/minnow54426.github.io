/* Image links remain usable without JavaScript; the dialog adds keyboard browsing. */
(function () {
  'use strict';
  const links = [...document.querySelectorAll('#main .thumb a.image')];
  if (!links.length || typeof HTMLDialogElement === 'undefined') return;
  const dialog = document.createElement('dialog');
  dialog.className = 'photo-viewer';
  dialog.setAttribute('aria-label', 'Photograph viewer');
  dialog.innerHTML = '<div class="photo-viewer__bar"><p aria-live="polite"></p><button type="button" data-close aria-label="Close photograph">Close</button></div><div class="photo-viewer__picture"><img alt=""></div><div class="photo-viewer__navigation"><button type="button" data-previous>Previous</button><span>Use ← and → to browse</span><button type="button" data-next>Next</button></div>';
  document.body.appendChild(dialog);
  const image = dialog.querySelector('img'), caption = dialog.querySelector('p');
  let current = 0, opener;
  function show(index) {
    current = (index + links.length) % links.length;
    const thumb = links[current].querySelector('img');
    image.src = links[current].href; image.alt = thumb.alt;
    caption.textContent = document.querySelector('.gallery-intro h2').textContent + ' — ' + (current+1) + ' of ' + links.length;
  }
  links.forEach((link,index) => link.addEventListener('click', event => {
    if (event.ctrlKey || event.metaKey || event.shiftKey || event.altKey) return;
    event.preventDefault();opener=link;show(index);dialog.showModal();document.body.classList.add('modal-active');
  }));
  dialog.querySelector('[data-close]').addEventListener('click', () => dialog.close());
  dialog.querySelector('[data-previous]').addEventListener('click', () => show(current-1));
  dialog.querySelector('[data-next]').addEventListener('click', () => show(current+1));
  dialog.addEventListener('keydown', event => {
    if(event.key==='ArrowLeft'){event.preventDefault();show(current-1);}
    if(event.key==='ArrowRight'){event.preventDefault();show(current+1);}
  });
  dialog.addEventListener('click', event => {if(event.target===dialog)dialog.close();});
  dialog.addEventListener('close', () => {document.body.classList.remove('modal-active');if(opener)opener.focus();});
})();
