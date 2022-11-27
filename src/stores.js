import {writable} from 'svelte/store';

export const notificationMsg = writable({
    type: '',
    message: '',
});

export const tableNames = writable({
    tableName: '',
    tables: [],
});

export const windowWidth = writable(0);
export const windowHeight = writable(0);

// setting state for one tab
export const activeTable = writable({
    tableName: '',
    rows: [[]],
    columns: [''],
    rowCount: 0,
    currentPage: 0,
    maxPage: 0
})