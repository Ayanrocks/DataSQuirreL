import { render, screen } from '@testing-library/svelte';
import { describe, it, expect } from 'vitest';
import InitScreen from './InitScreen.svelte';

describe('InitScreen Component', () => {
    it('renders correctly and queries recent projects', () => {
        const { container } = render(InitScreen);

        // Main connection layout contains both the ConnectForm constraints and the Recent projects area
        expect(screen.getByText('New Database Connection')).toBeInTheDocument();
        expect(screen.getByText('Recent Projects')).toBeInTheDocument();
        expect(container).toMatchSnapshot();
    });
});
