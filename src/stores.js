import { writable } from 'svelte/store';

export const notificationMsg = writable({
  type: '',
  message: '',
});
