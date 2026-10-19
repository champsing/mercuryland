/**
 * Height maths for `ViewportHeight.vue`.
 *
 * The wrapper is sized so that its bottom edge lines up with the bottom of the
 * viewport, which keeps the list area exactly one screen tall and lets the
 * inner scroll container do the scrolling.
 *
 * When the content above the wrapper (e.g. the penalty quick panels on a phone)
 * is taller than the viewport, that calculation goes negative. Clamping it to a
 * minimum keeps the list usable and lets the page itself scroll instead of
 * collapsing the list to nothing.
 */

/** Site footer height. */
export const FOOTNOTE_HEIGHT = 48;

/** Smallest usable list height (px). */
export const MIN_CONTENT_HEIGHT = 320;

export interface AvailableHeightOptions {
    /** `window.innerHeight`. */
    viewportHeight: number;
    /** `window.scrollY`. */
    scrollY: number;
    /** Wrapper's top edge, relative to the viewport (bounding rect). */
    top: number;
    footnoteHeight?: number;
    minHeight?: number;
}

export function availableHeight({
    viewportHeight,
    scrollY,
    top,
    footnoteHeight = FOOTNOTE_HEIGHT,
    minHeight = MIN_CONTENT_HEIGHT,
}: AvailableHeightOptions): number {
    const remaining = viewportHeight - scrollY - top - footnoteHeight;

    return Math.max(Math.round(remaining), minHeight);
}
