import { readStorage, writeStorage } from "@/composables/storage";
import { customRef, type Ref } from "vue";

/**
 * How a list of blocks (penalty / VOD) is laid out:
 * - `grid`: the original card layout, 3 cards per row on wide screens.
 * - `list`: a compact, table-like layout with one block per row.
 */
export type BlockViewMode = "grid" | "list";

export const BLOCK_VIEW_MODES: readonly BlockViewMode[] = ["grid", "list"];

const STORAGE_PREFIX = "block-view-mode:";

export function isBlockViewMode(value: unknown): value is BlockViewMode {
    return BLOCK_VIEW_MODES.includes(value as BlockViewMode);
}

export function blockViewModeStorageKey(key: string): string {
    return `${STORAGE_PREFIX}${key}`;
}

/** Read the persisted render mode for `key`, falling back when unavailable. */
export function readBlockViewMode(
    key: string,
    fallback: BlockViewMode = "grid",
): BlockViewMode {
    const stored = readStorage(blockViewModeStorageKey(key));
    return isBlockViewMode(stored) ? stored : fallback;
}

/** Persist the render mode for `key`; silently ignores storage failures. */
export function writeBlockViewMode(key: string, mode: BlockViewMode): void {
    writeStorage(blockViewModeStorageKey(key), mode);
}

/**
 * Two-way bound render mode that is remembered in `localStorage` under `key`,
 * so the user's choice survives reloads and page navigation.
 */
export function useBlockViewMode(
    key: string,
    fallback: BlockViewMode = "grid",
): Ref<BlockViewMode> {
    return customRef<BlockViewMode>((track, trigger) => {
        let current = readBlockViewMode(key, fallback);

        return {
            get() {
                track();
                return current;
            },
            set(value) {
                if (!isBlockViewMode(value) || value === current) return;
                current = value;
                writeBlockViewMode(key, value);
                trigger();
            },
        };
    });
}
