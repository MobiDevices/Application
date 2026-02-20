(() => {
    try {
        // Only run on our site.
        const host = String(window.location.hostname || '')
        if (!host || !/(^|\.)mobidevices\.com$/.test(host)) return

        const isInternal = (u) => {
            const h = u && u.hostname ? String(u.hostname) : ''
            return h === 'mobidevices.com' || h.endsWith('.mobidevices.com')
        }

        const shouldExternal = (u) => {
            if (!u) return false
            const scheme = String(u.protocol || '').replace(':', '')
            if (scheme === 'mailto' || scheme === 'tel') return true
            if (scheme === 'http' || scheme === 'https') return !isInternal(u)
            return true
        }

        document.addEventListener('click', (e) => {
            // Only handle normal primary-button clicks without modifiers.
            if (!e || e.defaultPrevented) return
            if (typeof e.button === 'number' && e.button !== 0) return
            if (e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return

            const el = e.target
            if (!el || !el.closest) return

            const a = el.closest('a, area')
            if (!a) return

            const href = a.getAttribute('href') || ''
            if (!href || href === '#') return

            let u
            try {
                u = new URL(href, window.location.href)
            } catch (_) {
                return
            }

            if (!shouldExternal(u)) return

            // Force external open path through Tauri's new-window handler.
            e.preventDefault()
            if (e.stopImmediatePropagation) e.stopImmediatePropagation()
            try {
                window.open(u.href, '_blank')
            } catch (_) {
                // If window.open is blocked for any reason, fallback to direct navigation.
                window.location.href = u.href
            }
        }, true)
    } catch (_) {}
})()
