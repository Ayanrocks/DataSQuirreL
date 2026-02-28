export type TransactionChangeType = "INSERT" | "UPDATE" | "DELETE";

export interface TransactionChange {
  type: TransactionChangeType;
  /** Primary keys of the row to uniquely identify it. Null for new inserts. */
  primaryKeys: Record<string, string> | null;
  /** Full original row data for fallback identification or comparison. */
  originalRow: Record<string, string> | null;
  /** The updated values for UPDATE, or the full new values for INSERT. Empty for DELETE. */
  newValues: Record<string, string> | null;
  /** Index of the row in the local table view (for UI reconciliation) */
  rowIndex?: number;
}

export class TransactionManager {
  // Keyed by row index for easy lookup during rendering
  private changes = new Map<number, TransactionChange>();

  public addRow(rowIndex: number, newRowData: Record<string, string>) {
    this.changes.set(rowIndex, {
      type: "INSERT",
      primaryKeys: null,
      originalRow: null,
      newValues: newRowData,
      rowIndex,
    });
  }

  public updateRow(
    rowIndex: number,
    primaryKeys: Record<string, string> | null,
    originalRow: Record<string, string>,
    newValues: Record<string, string>,
  ) {
    const existing = this.changes.get(rowIndex);

    if (existing) {
      if (existing.type === "INSERT") {
        // If it was an insert, just update the inserted values
        existing.newValues = { ...existing.newValues, ...newValues };
      } else if (existing.type === "UPDATE") {
        // Merge updates
        existing.newValues = { ...existing.newValues, ...newValues };
      }
      // If DELETE, we shouldn't be updating it
    } else {
      this.changes.set(rowIndex, {
        type: "UPDATE",
        primaryKeys,
        originalRow,
        newValues,
        rowIndex,
      });
    }
  }

  public deleteRow(
    rowIndex: number,
    primaryKeys: Record<string, string> | null,
    originalRow: Record<string, string>,
  ) {
    const existing = this.changes.get(rowIndex);
    if (existing && existing.type === "INSERT") {
      // If we are deleting a row we just inserted, simply cancel the insert
      this.changes.delete(rowIndex);
    } else {
      this.changes.set(rowIndex, {
        type: "DELETE",
        primaryKeys,
        originalRow,
        newValues: null,
        rowIndex,
      });
    }
  }

  public getChange(rowIndex: number): TransactionChange | undefined {
    return this.changes.get(rowIndex);
  }

  public getAllChanges(): TransactionChange[] {
    return Array.from(this.changes.values());
  }

  public hasChanges(): boolean {
    return this.changes.size > 0;
  }

  public clear() {
    this.changes.clear();
  }
}
