import { writable, type Writable } from 'svelte/store';

interface NotificationMsg {
    type: string;
    message: string;
}

export const notificationMsg: Writable<NotificationMsg> = writable({
    type: '',
    message: '',
});

interface TableNames {
    tableName: string;
    tables: string[];
}

export const tableNames: Writable<TableNames> = writable({
    tableName: '',
    tables: [],
});

export const windowWidth: Writable<number> = writable(0);
export const windowHeight: Writable<number> = writable(0);

// setting state for one tab
export interface ActiveTable {
    tableName: string;
    schemaName: string;
    dbName: string;
    displayName?: string;
    rows: string[][];
    columns: string[][];
    rowCount: number;
    currentPage: number;
    maxPage: number;
    currentOffset?: number;
    currentLimit?: number | null;
}

export const activeTable: Writable<ActiveTable> = writable({
    tableName: '',
    schemaName: '',
    dbName: '',
    rows: [[]],
    columns: [],
    rowCount: 0,
    currentPage: 0,
    maxPage: 0
})
