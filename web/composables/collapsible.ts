import { readStorage, writeStorage } from "@/composables/storage";
import { customRef, type Ref } from "vue";

const STORAGE_PREFIX = "collapsed:";

export function collapsedStorageKey(key: string): string {
    return `${STORAGE_PREFIX}${key}`;
}

/** Read the persisted collapsed flag for `key`, falling back when absent. */
export function readCollapsed(key: string, fallback = false): boolean {
    const stored = readStorage(collapsedStorageKey(key));
    if (stored === "true") return true;
    if (stored === "false") return false;
    return fallback;
}

/** Persist the collapsed flag for `key`. */
export function writeCollapsed(key: string, collapsed: boolean): void {
    writeStorage(collapsedStorageKey(key), collapsed ? "true" : "false");
}

/**
 * Two-way bound collapsed flag remembered in `localStorage`, so a section the
 * user folded away stays folded after a reload.
 */
export function useCollapsed(key: string, fallback = false): Ref<boolean> {
    return customRef<boolean>((track, trigger) => {
        let current = readCollapsed(key, fallback);

        return {
            get() {
                track();
                return current;
            },
            set(value) {
                if (typeof value !== "boolean" || value === current) return;
                current = value;
                writeCollapsed(key, value);
                trigger();
            },
        };
    });
}
