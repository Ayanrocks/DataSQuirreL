import Database from '@tauri-apps/plugin-sql';
import { compressSync, decompressSync } from 'fflate';
import { invoke } from '@tauri-apps/api/core';

export interface RowData {
  id: string;
  data: Record<string, any>;
}

export const db = await Database.load('sqlite:cache.db');

// Helper: compress JSON → Uint8Array
export function encodeRow(row: RowData): Uint8Array {
  const json = JSON.stringify(row);
  return compressSync(new TextEncoder().encode(json));
}

// Helper: decompress Uint8Array → RowData
export function decodeRow(blob: Uint8Array): RowData {
  const decompressed = decompressSync(blob);
  const str = new TextDecoder().decode(decompressed);
  return JSON.parse(str);
}

// Fetch and cache a chunk of rows from Rust
export async function fetchAndCache(
  tabId: string,
  offset: number,
  limit: number
): Promise<{ rows: RowData[]; total: number }> {
  // 1) fetch from DB
  const [rawRows, total] = await invoke< [RowData[], number] >(
    'fetch_table_rows',
    { tabId, offset, limit }
  );

  // 2) cache each row
  for (let i = 0; i < rawRows.length; i++) {
    const row = rawRows[i];
    const blob = encodeRow(row);
    await invoke('save_cache_entry', {
      tabId,
      rowIdx: offset + i,
      rowJson: JSON.stringify(row)
    });
  }

  return { rows: rawRows, total };
}

// Try loading one row from cache
export async function loadCachedRow(
  tabId: string,
  rowIdx: number
): Promise<RowData | null> {
  const result = await invoke<string | null>(
    'get_cache_entry',
    { tabId, rowIdx }
  );
  return result ? JSON.parse(result) as RowData : null;
}
