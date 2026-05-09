/* ============================================
   rslog 中文文档站 - PJAX + marked + highlight.js
   ============================================ */

(function() {

  var contentEl = document.getElementById('content');
  var loadingBar = document.getElementById('loading-bar');
  var sidebar = document.getElementById('sidebar');
  var currentMd = '1-INDEX.md';

  // ===== Configure marked =====
  marked.setOptions({
    highlight: false, // we use highlight.js separately
    gfm: true,
    breaks: false,
    langPrefix: 'language-',
  });

  // ===== Configure highlight.js =====
  hljs.configure({
    ignoreUnescapedHTML: true
  });

  // ===== Navigation function (exposed globally for logo click) =====
  window.navigate = function(mdFile, el) {
    loadPage(mdFile);
    if (el) return false; // prevent default for <a>
  };

  // ===== Load page via PJAX =====
  function loadPage(mdFile) {
    if (mdFile === currentMd) return;
    currentMd = mdFile;

    // Show loading
    loadingBar.style.opacity = '1';
    loadingBar.style.width = '30%';

    var xhr = new XMLHttpRequest();
    xhr.open('GET', mdFile, true);

    xhr.onprogress = function(e) {
      if (e.lengthComputable) {
        var pct = Math.min(70, 30 + (e.loaded / e.total) * 40);
        loadingBar.style.width = pct + '%';
      }
    };

    xhr.onload = function() {
      if (xhr.status === 200 || xhr.status === 0) {
        var md = xhr.responseText;

        // Handle relative links in md: replace .md references to HTML nav
        // We keep .md links as-is but they won't be clicked in the rendered content
        // (only sidebar links are used for navigation)

        var html = marked.parse(md);
        contentEl.innerHTML = html;

        // Highlight all code blocks
        contentEl.querySelectorAll('pre code').forEach(function(block) {
          hljs.highlightElement(block);
        });

        // Fix inline code colors after hljs
        contentEl.querySelectorAll('p code, li code, td code').forEach(function(el) {
          el.style.background = '#e8e8ee';
          el.style.color = '#c7254e';
        });

        // Add copy buttons
        addCopyButtons();

        // Update sidebar active
        updateSidebar(mdFile);

        // Update page title
        var titleEl = contentEl.querySelector('h1');
        document.title = (titleEl ? titleEl.textContent + ' - ' : '') + 'rslog 中文文档';

        // Complete loading
        loadingBar.style.width = '100%';
        setTimeout(function() {
          loadingBar.style.opacity = '0';
          loadingBar.style.width = '0';
        }, 300);

        // Scroll to top
        window.scrollTo({ top: 0, behavior: 'smooth' });

        // Close mobile sidebar
        if (window.innerWidth <= 768) {
          sidebar.classList.remove('open');
          document.getElementById('sidebar-overlay').classList.remove('active');
        }
      }
    };

    xhr.onerror = function() {
      contentEl.innerHTML = '<h1>加载失败</h1><p>无法加载文档内容，请检查网络连接后刷新页面。</p>';
      loadingBar.style.opacity = '0';
      loadingBar.style.width = '0';
    };

    xhr.send();

    // Update URL hash without triggering navigation
    history.replaceState({ md: mdFile }, '', '#' + mdFile);
  }

  // ===== Add copy buttons =====
  function addCopyButtons() {
    contentEl.querySelectorAll('pre').forEach(function(pre) {
      // Avoid duplicating buttons
      if (pre.querySelector('.copy-btn')) return;

      var btn = document.createElement('button');
      btn.className = 'copy-btn';
      btn.textContent = 'Copy';
      pre.appendChild(btn);

      btn.addEventListener('click', function(e) {
        e.stopPropagation();
        var code = pre.querySelector('code');
        var text = code ? (code.textContent || '') : '';

        if (navigator.clipboard && navigator.clipboard.writeText) {
          navigator.clipboard.writeText(text).then(function() {
            showCopied(btn);
          }).catch(function() {
            fallbackCopy(text, btn);
          });
        } else {
          fallbackCopy(text, btn);
        }
      });
    });
  }

  function fallbackCopy(text, btn) {
    var ta = document.createElement('textarea');
    ta.value = text;
    ta.style.position = 'fixed';
    ta.style.opacity = '0';
    document.body.appendChild(ta);
    ta.select();
    try {
      document.execCommand('copy');
      showCopied(btn);
    } catch (e) {}
    document.body.removeChild(ta);
  }

  function showCopied(btn) {
    var orig = btn.textContent;
    btn.textContent = 'Copied!';
    btn.classList.add('copied');
    setTimeout(function() {
      btn.textContent = orig;
      btn.classList.remove('copied');
    }, 2000);
  }

  // ===== Update sidebar active =====
  function updateSidebar(mdFile) {
    sidebar.querySelectorAll('.sidebar-link').forEach(function(link) {
      link.classList.remove('active');
      if (link.getAttribute('data-md') === mdFile) {
        link.classList.add('active');
      }
    });
  }

  // ===== Sidebar click handlers =====
  sidebar.querySelectorAll('.sidebar-link[data-md]').forEach(function(link) {
    link.addEventListener('click', function(e) {
      e.preventDefault();
      var md = this.getAttribute('data-md');
      if (md) loadPage(md);
    });
  });

  // ===== Mobile menu toggle =====
  var menuToggle = document.getElementById('menu-toggle');
  var overlay = document.getElementById('sidebar-overlay');

  if (menuToggle && sidebar && overlay) {
    function toggleMenu() {
      sidebar.classList.toggle('open');
      overlay.classList.toggle('active');
    }
    menuToggle.addEventListener('click', toggleMenu);
    overlay.addEventListener('click', toggleMenu);
  }

  // ===== Handle initial load =====
  // Check URL hash for which md to load
  var initialMd = location.hash.replace('#', '') || currentMd;
  if (initialMd && initialMd.match(/\.md$/)) {
    currentMd = ''; // force load
    loadPage(initialMd);
  } else {
    loadPage(currentMd);
  }

  // ===== Handle popstate (browser back/forward) =====
  window.addEventListener('popstate', function(e) {
    if (e.state && e.state.md) {
      var md = e.state.md;
      // Temporarily set currentMd to force reload
      var prev = currentMd;
      currentMd = md + '_force'; // different string to force reload
      loadPage(md);
    }
  });

})();
