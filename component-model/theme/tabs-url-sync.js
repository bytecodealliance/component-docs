(function () {
    'use strict';

    const GLOBAL_NAME = 'wasi-version';
    const PARAM_NAME = 'wasi';
    const SLUG_TO_NAME = { p2: 'WASI P2', p3: 'WASI P3' };
    const NAME_TO_SLUG = { 'WASI P2': 'p2', 'WASI P3': 'p3' };

    function readSlugFromUrl() {
        const value = new URLSearchParams(window.location.search).get(PARAM_NAME);
        return value && SLUG_TO_NAME[value] ? value : null;
    }

    function writeSlugToUrl(slug) {
        const url = new URL(window.location.href);
        url.searchParams.set(PARAM_NAME, slug);
        history.replaceState(null, '', url.toString());
    }

    function activateByName(name) {
        const tab = document.querySelector(
            '.mdbook-tabs-container[data-tabglobal="' + GLOBAL_NAME + '"] ' +
            '.mdbook-tab[data-tabname="' + name + '"]'
        );
        if (tab) {
            tab.click();
        }
    }

    // If `element` lives inside a tab pane, activate that pane's tab.
    // Returns true if a tab was activated.
    function activateTabContainingElement(element) {
        const pane = element.closest && element.closest('.mdbook-tab-content');
        if (!pane) return false;
        const container = pane.closest('.mdbook-tabs-container');
        if (!container) return false;
        const name = pane.dataset.tabname;
        if (!name) return false;
        const tab = container.querySelector('.mdbook-tab[data-tabname="' + name + '"]');
        if (!tab) return false;
        tab.click();
        return true;
    }

    document.addEventListener('DOMContentLoaded', function () {
        // Apply URL-param tab selection first.
        const slug = readSlugFromUrl();
        if (slug) {
            activateByName(SLUG_TO_NAME[slug]);
        }

        // If the page was loaded with a hash pointing to an element inside a
        // hidden tab pane, activate that pane and re-scroll. Hash wins over
        // the ?wasi= param since the hash targets a specific element.
        if (window.location.hash.length > 1) {
            const targetId = decodeURIComponent(window.location.hash.slice(1));
            const target = document.getElementById(targetId);
            if (target && activateTabContainingElement(target)) {
                target.scrollIntoView();
            }
        }

        // Reflect wasi-version tab clicks back into the URL.
        document.addEventListener('click', function (event) {
            const tab = event.target.closest && event.target.closest('.mdbook-tab');
            if (!tab) return;
            const container = tab.closest('.mdbook-tabs-container');
            if (!container || container.dataset.tabglobal !== GLOBAL_NAME) return;
            const newSlug = NAME_TO_SLUG[tab.dataset.tabname];
            if (newSlug) {
                writeSlugToUrl(newSlug);
            }
        });
    });
})();
