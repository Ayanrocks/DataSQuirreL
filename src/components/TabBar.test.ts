import { render, screen, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import TabBar from './TabBar.svelte';

describe('TabBar Component', () => {
    it('renders tabs correctly', () => {
        const mockTabs = [
            { id: '1', displayName: 'Tab One', tableName: 'tab1' },
            { id: '2', displayName: 'Tab Two', tableName: 'tab2' },
        ] as any;

        const { container } = render(TabBar, {
            tabs: mockTabs,
            activeTabIndex: 0,
            onTabChange: vi.fn(),
            onTabClose: vi.fn()
        });

        expect(screen.getByText('Tab One')).toBeInTheDocument();
        expect(screen.getByText('Tab Two')).toBeInTheDocument();
        expect(container).toMatchSnapshot();
    });

    it('calls functions on click', async () => {
        const mockTabs = [
            { id: '1', displayName: 'Tab One' },
        ] as any;

        const onTabChange = vi.fn();
        const onTabClose = vi.fn();

        render(TabBar, {
            tabs: mockTabs,
            activeTabIndex: 0,
            onTabChange,
            onTabClose
        });

        const tabLabel = screen.getByText('Tab One');
        await fireEvent.click(tabLabel);
        expect(onTabChange).toHaveBeenCalledWith(0);

        const closeBtn = screen.getByLabelText('Close tab');
        await fireEvent.click(closeBtn);
        expect(onTabClose).toHaveBeenCalledWith(0);
    });
});
