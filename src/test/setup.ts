import '@testing-library/jest-dom/vitest';
import { vi } from 'vitest';

// Mock matchMedia for jsdom
Object.defineProperty(window, 'matchMedia', {
    writable: true,
    value: vi.fn().mockImplementation(query => ({
        matches: false,
        media: query,
        onchange: null,
        addListener: vi.fn(), // deprecated
        removeListener: vi.fn(), // deprecated
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
    })),
});

// Mock ResizeObserver for jsdom
globalThis.ResizeObserver = class ResizeObserver {
    observe() { }
    unobserve() { }
    disconnect() { }
};

// Mock Web Animations API for svelte transitions
vi.mock('svelte/transition', () => ({
    slide: () => ({}),
    fade: () => ({}),
    fly: () => ({}),
    scale: () => ({}),
    draw: () => ({}),
    crossfade: () => [() => ({}), () => ({})],
}));

if (!Element.prototype.animate) {
    Element.prototype.animate = vi.fn().mockImplementation(() => ({
        finished: Promise.resolve(),
        cancel: vi.fn(),
        play: vi.fn(),
        pause: vi.fn(),
    }));
}

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => {
    return {
        invoke: vi.fn().mockResolvedValue([]),
        isTauri: () => true,
    };
});

// @ts-ignore
window.__TAURI_INTERNALS__ = {};

// Mock other common Tauri plugins if needed
vi.mock('@tauri-apps/plugin-clipboard-manager', () => ({
    writeText: vi.fn().mockResolvedValue(undefined),
    readText: vi.fn().mockResolvedValue(''),
}));

vi.mock('@tauri-apps/api/window', () => ({
    getCurrentWindow: vi.fn(() => ({
        label: Promise.resolve('connection-window-test'),
        close: vi.fn(),
        minimize: vi.fn(),
        toggleMaximize: vi.fn(),
        isMaximized: vi.fn().mockResolvedValue(false),
        onResized: vi.fn().mockResolvedValue(vi.fn()),
        scaleFactor: vi.fn().mockResolvedValue(1),
        innerSize: vi.fn().mockResolvedValue({
            toLogical: vi.fn(() => ({ width: 1024, height: 768 }))
        })
    })),
}));

vi.mock('@tauri-apps/plugin-os', () => ({
    platform: vi.fn().mockResolvedValue('macos'),
}));
