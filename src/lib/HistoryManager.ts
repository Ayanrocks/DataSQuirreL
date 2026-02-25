export interface HistoryOp {
    r: number;
    c: number;
    oldVal: string;
    newVal: string;
}

export class HistoryManager {
    private undoStack: HistoryOp[][] = [];
    private redoStack: HistoryOp[][] = [];

    constructor() { }

    push(ops: HistoryOp[]) {
        if (ops.length === 0) return;
        this.undoStack.push(ops);
        this.redoStack = []; // Clear redo stack on new action
    }

    undo(rows: string[][]): boolean {
        if (this.undoStack.length === 0) return false;
        const ops = this.undoStack.pop()!;

        // Apply old values
        for (const op of ops) {
            if (rows[op.r] && rows[op.r].length >= op.c) {
                rows[op.r][op.c - 1] = op.oldVal;
            }
        }

        this.redoStack.push(ops);
        return true;
    }

    redo(rows: string[][]): boolean {
        if (this.redoStack.length === 0) return false;
        const ops = this.redoStack.pop()!;

        // Apply new values
        for (const op of ops) {
            if (rows[op.r] && rows[op.r].length >= op.c) {
                rows[op.r][op.c - 1] = op.newVal;
            }
        }

        this.undoStack.push(ops);
        return true;
    }

    clear() {
        this.undoStack = [];
        this.redoStack = [];
    }
}
