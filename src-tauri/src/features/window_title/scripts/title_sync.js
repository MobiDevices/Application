(() => {
    try {
        const invoke = window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke
        if (typeof invoke !== 'function') return

        let lastTitle = ''

        const syncTitle = (force) => {
            try {
                const currentTitle = String(document.title || '').trim()
                if (!currentTitle) return
                if (!force && currentTitle === lastTitle) return
                lastTitle = currentTitle
                invoke('set_window_title', { title: currentTitle }).catch(() => {})
            } catch (_) {}
        }

        const forceSyncSoon = () => {
            syncTitle(true)
            setTimeout(() => {
                syncTitle(true)
            }, 0)
        }

        forceSyncSoon()

        document.addEventListener('DOMContentLoaded', () => {
            forceSyncSoon()
        })
        window.addEventListener('load', () => {
            forceSyncSoon()
        })
        window.addEventListener('pageshow', () => {
            forceSyncSoon()
        })
        window.addEventListener('popstate', () => {
            forceSyncSoon()
        })
        window.addEventListener('hashchange', () => {
            forceSyncSoon()
        })

        if (document && document.head && typeof MutationObserver !== 'undefined') {
            const titleEl = document.head.querySelector('title')
            if (titleEl) {
                const observer = new MutationObserver(() => {
                    syncTitle(false)
                })
                observer.observe(titleEl, {
                    subtree: true,
                    characterData: true,
                    childList: true,
                })
            }
        }
    } catch (_) {}
})()
