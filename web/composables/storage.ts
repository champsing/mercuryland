/**
 * Tiny `localStorage` helpers that never throw: private mode, disabled
 * storage, or a full quota should degrade to "preference not remembered"
 * instead of breaking the page.
 */

export function readStorage(key: string): string | null {
    try {
        return localStorage.getItem(key);
    } catch {
        return null;
    }
}

export function writeStorage(key: string, value: string): void {
    try {
        localStorage.setItem(key, value);
    } catch {
        // Ignored on purpose — the UI keeps working in memory.
    }
}
