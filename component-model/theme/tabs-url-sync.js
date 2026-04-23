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

    document.addEventListener('DOMContentLoaded', function () {
        const slug = readSlugFromUrl();
        if (slug) {
            activateByName(SLUG_TO_NAME[slug]);
        }

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
