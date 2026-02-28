import { render, screen } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import Sidebar from './Sidebar.svelte';

describe('Sidebar Component', () => {
    it('renders loading state when dashboard data is not present', () => {
        const { container } = render(Sidebar, {
            dashboardData: undefined as any,
            onTableSelect: vi.fn()
        });

        expect(screen.getByText('Loading...')).toBeInTheDocument();
    });

    it('renders structured data', () => {
        const mockData = [
            {
                entityName: 'mydb',
                isExpanded: true,
                entityType: 'postgresql',
                children: [
                    {
                        entityName: 'public',
                        isExpanded: true,
                        entityType: 'Schema',
                        children: [
                            {
                                entityName: 'logs',
                                isExpanded: false,
                                entityType: 'Table',
                                children: []
                            }
                        ]
                    }
                ]
            }
        ] as any;

        const { container } = render(Sidebar, {
            // @ts-ignore
            dashboardData: mockData,
            onTableSelect: vi.fn()
        });

        expect(screen.getByText('mydb')).toBeInTheDocument();
        expect(screen.getByText('public')).toBeInTheDocument();
        expect(screen.getByText('logs')).toBeInTheDocument();
        expect(container).toMatchSnapshot();
    });
});
