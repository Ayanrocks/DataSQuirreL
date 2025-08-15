import { writable } from 'svelte/store';
import { fetchAndCache, loadCachedRow } from '../cache';
function createTabs() {
    const { subscribe, update } = writable([]);
    return {
        subscribe,
        async openTable(tabId, title, pageSize = 200) {
            const newTab = {
                id: tabId,
                title,
                type: 'table',
                rows: [],
                totalRows: 0,
                pageSize,
                offset: 0,
                isLoading: true
            };
            update(tabs => [...tabs, newTab]);
            // load first chunk
            const { rows, total } = await fetchAndCache(tabId, 0, pageSize);
            update(tabs => tabs.map(t => t.id === tabId
                ? Object.assign(Object.assign({}, t), { rows, totalRows: total, isLoading: false }) : t));
        },
        async loadMore(tabId) {
            let stateToUse;
            update(tabs => {
                const updatedTabs = tabs.map(t => {
                    if (t.id === tabId) {
                        stateToUse = Object.assign(Object.assign({}, t), { isLoading: true });
                        return stateToUse;
                    }
                    return t;
                });
                return updatedTabs;
            });
            if (!stateToUse) {
                console.error(`Tab with ID ${tabId} not found during loadMore.`);
                return;
            }
            const nextOffset = stateToUse.offset + stateToUse.pageSize;
            // first try cache
            const cachedRows = [];
            for (let i = nextOffset; i < nextOffset + stateToUse.pageSize; i++) {
                const r = await loadCachedRow(tabId, i);
                if (!r)
                    break;
                cachedRows.push(r);
            }
            if (cachedRows.length === stateToUse.pageSize) {
                // full page in cache
                update(tabs => tabs.map(t => t.id === tabId
                    ? Object.assign(Object.assign({}, t), { rows: [...t.rows, ...cachedRows], offset: nextOffset, isLoading: false }) : t));
            }
            else {
                // fetch fresh and cache
                const { rows } = await fetchAndCache(tabId, nextOffset, stateToUse.pageSize);
                update(tabs => tabs.map(t => t.id === tabId
                    ? Object.assign(Object.assign({}, t), { rows: [...t.rows, ...rows], offset: nextOffset, isLoading: false }) : t));
            }
        }
    };
}
export const tabs = createTabs();
//# sourceMappingURL=tabs.js.map