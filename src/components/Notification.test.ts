import { render, screen, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, beforeEach } from 'vitest';
import { tick } from 'svelte';
import Notification from './Notification.svelte';
import { notificationMsg } from '../stores';
import { NOTIFICATION_TYPE_ERROR, NOTIFICATION_TYPE_SUCCESS } from '../constants/constants';

describe('Notification Component', () => {
    beforeEach(() => {
        // Reset store
        notificationMsg.set({ type: '', message: '' });
    });

    it('renders nothing when empty', () => {
        render(Notification);
        const notifications = document.querySelectorAll('.notification');
        expect(notifications.length).toBe(0);
    });

    it('renders success notification then closes on click', async () => {
        render(Notification);

        // trigger state
        notificationMsg.set({ type: NOTIFICATION_TYPE_SUCCESS, message: 'Saved successfully!' });
        await tick();

        const notificationBody = screen.getByText('Saved successfully!');
        expect(notificationBody).toBeInTheDocument();
        expect(notificationBody.parentElement).toHaveClass('is-primary');

        // click close
        const closeBtn = screen.getByRole('button', { name: /close notification/i });
        await fireEvent.click(closeBtn);
        await tick();

        // Notification component removes it immediately from DOM as soon as hideMsg triggers store update
        expect(screen.queryByText('Saved successfully!')).not.toBeInTheDocument();
    });

    it('renders error notification', async () => {
        render(Notification);

        notificationMsg.set({ type: NOTIFICATION_TYPE_ERROR, message: 'Failed to connect.' });
        await tick();

        const notificationBody = screen.getByText('Failed to connect.');
        expect(notificationBody).toBeInTheDocument();
        expect(notificationBody.parentElement).toHaveClass('is-danger');
    });
});
