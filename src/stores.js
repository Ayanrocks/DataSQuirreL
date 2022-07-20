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