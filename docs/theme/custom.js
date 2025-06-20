// Custom JavaScript for Reservoir Documentation
// Disable theme switching and force system preference detection

(function() {
    'use strict';

    // Remove theme switching functionality
    function disableThemeSwitching() {
        // Remove theme toggle button
        const themeToggle = document.getElementById('theme-toggle');
        if (themeToggle) {
            themeToggle.remove();
        }

        // Remove theme popup
        const themePopup = document.querySelector('.theme-popup');
        if (themePopup) {
            themePopup.remove();
        }

        // Remove any theme-related event listeners
        document.removeEventListener('keydown', handleThemeToggle);
    }

    // Force system theme detection
    function enforceSystemTheme() {
        // Remove any stored theme preferences
        localStorage.removeItem('mdbook-theme');
        sessionStorage.removeItem('mdbook-theme');

        // Force body to use system colors
        document.body.style.backgroundColor = 'var(--bg-color)';
        document.body.style.color = 'var(--text-color)';

        // Remove any theme classes from html/body
        const themeClasses = ['light', 'coal', 'navy', 'ayu', 'rust'];
        themeClasses.forEach(cls => {
            document.documentElement.classList.remove(cls);
            document.body.classList.remove(cls);
        });

        // Add a custom class to indicate we're using system theme
        document.documentElement.classList.add('system-theme');
    }

    // Apply system theme based on media query
    function applySystemTheme() {
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

        // Add appropriate class for debugging/styling purposes
        if (prefersDark) {
            document.documentElement.classList.add('system-dark');
            document.documentElement.classList.remove('system-light');
        } else {
            document.documentElement.classList.add('system-light');
            document.documentElement.classList.remove('system-dark');
        }
    }

    // Listen for system theme changes
    function watchSystemTheme() {
        const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

        // Apply initial theme
        applySystemTheme();

        // Listen for changes
        mediaQuery.addListener(applySystemTheme);

        // Modern browsers
        if (mediaQuery.addEventListener) {
            mediaQuery.addEventListener('change', applySystemTheme);
        }
    }

    // Prevent mdBook from setting themes
    function interceptThemeStorage() {
        const originalSetItem = Storage.prototype.setItem;
        Storage.prototype.setItem = function(key, value) {
            // Block any theme-related storage
            if (key === 'mdbook-theme' || key.includes('theme')) {
                return;
            }
            return originalSetItem.call(this, key, value);
        };
    }

    // Override mdBook's theme functions
    function overrideThemeFunctions() {
        // Override any global theme functions that might exist
        if (window.theme) {
            window.theme = {
                set: function() { return; },
                get: function() { return 'system'; },
                toggle: function() { return; }
            };
        }

        // Prevent theme changes via URL parameters
        if (window.location.search.includes('theme=')) {
            const url = new URL(window.location);
            url.searchParams.delete('theme');
            window.history.replaceState({}, document.title, url.toString());
        }
    }

    // Convert title to a proper link
    function makeTitleClickable() {
        const title = document.querySelector('.menu-title');
        if (title) {
            // Create a link element
            const link = document.createElement('a');
            link.href = '/';
            link.textContent = title.textContent;
            link.className = title.className;
            
            // Replace the title with the link
            title.parentNode.replaceChild(link, title);
        }
    }

    // Initialize everything when DOM is ready
    function initialize() {
        // Disable theme switching
        disableThemeSwitching();

        // Enforce system theme
        enforceSystemTheme();

        // Watch for system theme changes
        watchSystemTheme();

        // Intercept storage attempts
        interceptThemeStorage();

        // Override theme functions
        overrideThemeFunctions();

        // Make title clickable
        makeTitleClickable();

        console.log('Reservoir documentation: System theme mode active');
    }

    // Run initialization
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initialize);
    } else {
        initialize();
    }

    // Also run on page navigation (for SPA-like behavior)
    window.addEventListener('popstate', initialize);

    // Watch for dynamic content changes
    const observer = new MutationObserver(function(mutations) {
        mutations.forEach(function(mutation) {
            if (mutation.type === 'childList') {
                // Re-run theme enforcement if new elements are added
                const themeToggle = document.getElementById('theme-toggle');
                if (themeToggle) {
                    disableThemeSwitching();
                }
            }
        });
    });

    // Start observing
    observer.observe(document.body, {
        childList: true,
        subtree: true
    });

})();
