/**
 * Returns the index a tablist should move to for a keyboard event, or `null`
 * when the key is not a tablist navigation key.
 *
 * Horizontal and vertical arrows are both supported so the same helper works
 * for the page tabs and for the legend's compact tab selectors. Navigation
 * wraps at both ends, matching the WAI-ARIA tabs pattern.
 */
export function nextTabIndex(
    key: string,
    current: number,
    count: number,
): number | null {
    if (count <= 0 || current < 0 || current >= count) return null;

    switch (key) {
        case "ArrowRight":
        case "ArrowDown":
            return (current + 1) % count;
        case "ArrowLeft":
        case "ArrowUp":
            return (current - 1 + count) % count;
        case "Home":
            return 0;
        case "End":
            return count - 1;
        default:
            return null;
    }
}
