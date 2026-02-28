import { render, screen } from '@testing-library/svelte';
import { describe, it, expect } from 'vitest';
import Loader from './Loader.svelte';

describe('Loader Component', () => {
    it('does not render when loaderActive is false', () => {
        const { container } = render(Loader, {
            loaderActive: false,
        });
        const loaderElement = container.querySelector('.lds-dual-ring');
        expect(loaderElement).not.toBeInTheDocument();
    });

    it('renders and applies styles when loaderActive is true', () => {
        const { container } = render(Loader, {
            loaderActive: true,
            height: '30px',
            width: '30px',
            color: '#ff0000',
        });

        const loaderElement = container.querySelector('.lds-dual-ring') as HTMLElement;
        expect(loaderElement).toBeInTheDocument();

        // Check computed styles matching the custom properties
        expect(loaderElement.style.getPropertyValue('height')).toBe('30px');
        expect(loaderElement.style.getPropertyValue('--width')).toBe('30px');
        expect(loaderElement.style.getPropertyValue('--color')).toBe('#ff0000');

        expect(container).toMatchSnapshot();
    });
});
