import { render, screen } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import RecursiveSidebarItem from './RecursiveSidebarItem.svelte';

describe('RecursiveSidebarItem Component', () => {
    it('renders expanded item and its children', () => {
        const mockItem = {
            entityName: 'public',
            isExpanded: true,
            entityType: 'Schema',
            level: 0,
            children: [
                {
                    entityName: 'users',
                    isExpanded: false,
                    entityType: 'Table',
                    level: 1,
                    children: []
                },
            ]
        };

        const { container } = render(RecursiveSidebarItem, {
            item: mockItem,
            parentContext: { databaseName: 'mydb' },
            handleTableClick: vi.fn()
        });

        expect(screen.getByText('public')).toBeInTheDocument();
        // Because it's expanded and has children, the child should also render
        expect(screen.getByText('users')).toBeInTheDocument();
        expect(container).toMatchSnapshot();
    });
});
