import { type Writable, writable } from 'svelte/store';
import { type IActiveTable } from './types/interface';

export const notificationMsg = writable({
  type: '',
  message: '',
});

export const tableNames: Writable<{
  tableName: string;
  tables: string[];
}> = writable({
  tableName: '',
  tables: [],
});

export const windowWidth: Writable<number> = writable(0);
export const windowHeight = writable(0);

// setting state for one tab
export const activeTable: Writable<IActiveTable> = writable({
  tableName: '',
  rows: [[]],
  columns: [''],
  rowCount: 0,
  currentPage: 0,
  maxPage: 0,
});
