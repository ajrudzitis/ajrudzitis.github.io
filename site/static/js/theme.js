// Theme switching logic
(function() {
    'use strict';

    const THEMES = ['minimalist', 'terminal', 'brutalist', 'postmodern', 'evergreen', 'autumn', 'correspondence', 'zen', 'workshop'];
    const STORAGE_KEY = 'site-theme';

    function getCurrentTheme() {
        return localStorage.getItem(STORAGE_KEY) ||
               document.documentElement.dataset.defaultTheme ||
               'minimalist';
    }

    function setTheme(theme) {
        const themeLink = document.getElementById('theme-css');
        if (themeLink) {
            themeLink.href = `/css/themes/${theme}.css`;
        }
        document.documentElement.dataset.theme = theme;
        localStorage.setItem(STORAGE_KEY, theme);
    }

    function cycleTheme() {
        const current = getCurrentTheme();
        const currentIndex = THEMES.indexOf(current);
        const nextIndex = (currentIndex + 1) % THEMES.length;
        const nextTheme = THEMES[nextIndex];

        setTheme(nextTheme);

        // Brief visual feedback
        showThemeNotification(nextTheme);
    }

    function showThemeNotification(theme) {
        // Remove existing notification if any
        const existing = document.getElementById('theme-notification');
        if (existing) {
            existing.remove();
        }

        const notification = document.createElement('div');
        notification.id = 'theme-notification';
        notification.textContent = theme;
        notification.style.cssText = `
            position: fixed;
            bottom: 20px;
            right: 20px;
            padding: 12px 24px;
            background: rgba(0, 0, 0, 0.8);
            color: white;
            font-family: system-ui, sans-serif;
            font-size: 14px;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 2px;
            border-radius: 4px;
            z-index: 10000;
            opacity: 0;
            transform: translateY(10px);
            transition: opacity 0.2s, transform 0.2s;
        `;

        document.body.appendChild(notification);

        // Trigger animation
        requestAnimationFrame(() => {
            notification.style.opacity = '1';
            notification.style.transform = 'translateY(0)';
        });

        // Remove after delay
        setTimeout(() => {
            notification.style.opacity = '0';
            notification.style.transform = 'translateY(10px)';
            setTimeout(() => notification.remove(), 200);
        }, 1500);
    }

    // Secret sequence: type "theme" to cycle themes
    const themeSequence = ['t', 'h', 'e', 'm', 'e'];
    let themeIndex = 0;
    let themeTimeout;

    document.addEventListener('keydown', function(e) {
        // Ignore if user is typing in an input field
        if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA' || e.target.isContentEditable) {
            return;
        }

        // Reset sequence after 2 seconds of no input
        clearTimeout(themeTimeout);
        themeTimeout = setTimeout(() => { themeIndex = 0; }, 2000);

        if (e.key.toLowerCase() === themeSequence[themeIndex]) {
            themeIndex++;
            if (themeIndex === themeSequence.length) {
                themeIndex = 0;
                cycleTheme();
            }
        } else {
            themeIndex = 0;
        }
    });

    // Initialize theme on DOM ready (backup for inline script)
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', function() {
            setTheme(getCurrentTheme());
        });
    }

    // Expose for potential programmatic use
    window.siteTheme = {
        get: getCurrentTheme,
        set: setTheme,
        cycle: cycleTheme,
        themes: THEMES
    };

    // Console hint
    console.log('%c🎨 Type "theme" to cycle through visual styles', 'color: #6366f1; font-weight: bold;');
})();
