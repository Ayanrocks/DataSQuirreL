import { render, screen, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import DataTableToolBar from './DataTableToolBar.svelte';
import { activeTable } from '../stores';

describe('DataTableToolBar Component', () => {
    it('renders correctly', () => {
        const { container } = render(DataTableToolBar);
        expect(screen.getByLabelText('Add Row')).toBeInTheDocument();
        expect(screen.getByLabelText('Delete Row')).toBeInTheDocument();
        expect(screen.getByLabelText('Commit Changes')).toBeInTheDocument();
        expect(screen.getByLabelText('Refresh')).toBeInTheDocument();
        expect(container).toMatchSnapshot();
    });

    it('calls onAddRow when add button is clicked', async () => {
        const onAddRow = vi.fn();
        render(DataTableToolBar, {
            onAddRow,
        });
        const addBtn = screen.getByLabelText('Add Row');
        await fireEvent.click(addBtn);
        expect(onAddRow).toHaveBeenCalled();
    });

    it('calls onRemoveRow when delete button is clicked', async () => {
        const onRemoveRow = vi.fn();
        render(DataTableToolBar, {
            onRemoveRow,
        });
        const deleteBtn = screen.getByLabelText('Delete Row');
        await fireEvent.click(deleteBtn);
        expect(onRemoveRow).toHaveBeenCalled();
    });

    it('calls onRefresh when refresh button is clicked', async () => {
        const onRefresh = vi.fn();
        render(DataTableToolBar, {
            onRefresh,
        });
        const refreshBtn = screen.getByLabelText('Refresh');
        await fireEvent.click(refreshBtn);
        expect(onRefresh).toHaveBeenCalled();
    });

    it('handles limit selection change', async () => {
        const onLimitChange = vi.fn();
        render(DataTableToolBar, {
            onLimitChange,
        });
        const select = screen.getByRole('combobox');
        await fireEvent.change(select, { target: { value: '1000' } });
        expect(onLimitChange).toHaveBeenCalledWith(1000);
    });

    it('disables prev/first pagination back buttons when currentPage is 1', () => {
        render(DataTableToolBar, {
            currentPage: 1,
            maxPage: 5
        });

        const firstBtn = screen.getByLabelText('First Page') as HTMLButtonElement;
        const prevBtn = screen.getByLabelText('Previous Page') as HTMLButtonElement;

        expect(firstBtn.disabled).toBe(true);
        expect(prevBtn.disabled).toBe(true);
    });
});
