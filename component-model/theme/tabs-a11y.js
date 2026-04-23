(function () {
    'use strict';

    // Layer ARIA semantics and keyboard navigation onto the mdbook-tabs
    // plugin's markup. The plugin emits plain <button> / <div> elements
    // without tab/tablist/tabpanel roles; this script fills that gap.

    let uidCounter = 0;
    function nextUid() {
        uidCounter += 1;
        return uidCounter;
    }

    function initContainer(container) {
        const nav = container.querySelector('.mdbook-tabs');
        if (nav && !nav.hasAttribute('role')) {
            nav.setAttribute('role', 'tablist');
        }

        const tabs = Array.from(container.querySelectorAll('.mdbook-tab'));
        const panes = Array.from(container.querySelectorAll('.mdbook-tab-content'));

        tabs.forEach(function (tab) {
            if (tab.hasAttribute('role')) return;
            const uid = nextUid();
            const tabId = 'mdbook-tab-' + uid;
            const paneId = 'mdbook-tabpanel-' + uid;

            tab.setAttribute('role', 'tab');
            tab.id = tabId;

            const pane = panes.find(function (p) {
                return p.dataset.tabname === tab.dataset.tabname;
            });
            if (pane) {
                pane.setAttribute('role', 'tabpanel');
                pane.id = paneId;
                if (!pane.hasAttribute('tabindex')) {
                    pane.setAttribute('tabindex', '0');
                }
                tab.setAttribute('aria-controls', paneId);
                pane.setAttribute('aria-labelledby', tabId);
            }
        });
    }

    function syncContainer(container) {
        container.querySelectorAll('.mdbook-tab').forEach(function (tab) {
            const isActive = tab.classList.contains('active');
            tab.setAttribute('aria-selected', isActive ? 'true' : 'false');
            // Roving tabindex: only the active tab is reachable via Tab key;
            // arrow keys move focus within the tablist.
            tab.setAttribute('tabindex', isActive ? '0' : '-1');
        });
        container.querySelectorAll('.mdbook-tab-content').forEach(function (pane) {
            pane.setAttribute('aria-hidden', pane.classList.contains('hidden') ? 'true' : 'false');
        });
    }

    function syncAll() {
        document.querySelectorAll('.mdbook-tabs-container').forEach(syncContainer);
    }

    function handleKeyDown(event) {
        const tab = event.target.closest && event.target.closest('.mdbook-tab');
        if (!tab) return;
        const nav = tab.parentElement;
        if (!nav || !nav.classList.contains('mdbook-tabs')) return;

        const tabs = Array.from(nav.querySelectorAll('.mdbook-tab'));
        const currentIndex = tabs.indexOf(tab);
        if (currentIndex < 0) return;

        let newIndex = null;
        switch (event.key) {
            case 'ArrowRight':
            case 'Right':
                newIndex = (currentIndex + 1) % tabs.length;
                break;
            case 'ArrowLeft':
            case 'Left':
                newIndex = (currentIndex - 1 + tabs.length) % tabs.length;
                break;
            case 'Home':
                newIndex = 0;
                break;
            case 'End':
                newIndex = tabs.length - 1;
                break;
        }
        if (newIndex === null) return;

        event.preventDefault();
        tabs[newIndex].focus();
        // Automatic activation: mirrors common tab widgets (Bootstrap, Radix,
        // etc.) where focusing a tab also selects it.
        tabs[newIndex].click();
    }

    document.addEventListener('DOMContentLoaded', function () {
        document.querySelectorAll('.mdbook-tabs-container').forEach(initContainer);
        syncAll();

        document.addEventListener('keydown', handleKeyDown);

        // Re-sync ARIA state after every click. Clicks may propagate across
        // multiple containers when a global="..." state is shared, so we
        // resync every container, not just the one that was clicked.
        document.addEventListener('click', function (event) {
            if (!event.target.closest || !event.target.closest('.mdbook-tab')) return;
            syncAll();
        });
    });
})();
