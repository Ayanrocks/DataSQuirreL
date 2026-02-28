import { render, screen } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import DataTable from './DataTable.svelte';

// Mock matchMedia if not already in setup.ts
if (typeof window.matchMedia !== 'function') {
    window.matchMedia = vi.fn().mockImplementation((query) => ({
        matches: false,
        media: query,
        onchange: null,
        addListener: vi.fn(),
        removeListener: vi.fn(),
    }));
}

describe('DataTable Component', () => {
    it('renders successfully without crashing', () => {
        const mockTableData = {
            id: 'test_db::public::users',
            tableName: 'users',
            schemaName: 'public',
            dbName: 'test_db',
            displayName: 'users',
            columns: [
                ['id', 'id', 'integer'],
                ['name', 'name', 'text'],
            ],
            rows: [
                ['1', 'Alice'],
                ['2', 'Bob'],
            ],
            currentPage: 1,
            maxPage: 1,
            rowCount: 2,
        };

        const { container } = render(DataTable, {
            activeTableData: mockTableData,
            fetchData: vi.fn(),
        });

        // Check if toolbar rendered by checking one of its known labels
        expect(screen.getByLabelText('Add Row')).toBeInTheDocument();

        // Check if table rendered header cells
        expect(screen.getByText('id')).toBeInTheDocument();
        expect(screen.getByText('name')).toBeInTheDocument();

        // Check if data rows rendered
        expect(screen.getByText('Alice')).toBeInTheDocument();
        expect(screen.getByText('Bob')).toBeInTheDocument();

        expect(container).toMatchSnapshot();
    });
});
