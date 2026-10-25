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
    /** `window.innerHeight`, or the bottom edge of a scroll container. */
    viewportHeight: number;
    /** Scroll offset: `window.scrollY` or a container's `scrollTop`. */
    scrollY: number;
    /** Wrapper's top edge, relative to the viewport (bounding rect). */
    top: number;
    footnoteHeight?: number;
    /**
     * Height added back before clamping (a collapsible panel above the area).
     * Adding the panel height back keeps the measured area stable when a
     * collapsible panel above it opens.
     */
    extraHeight?: number;
    minHeight?: number;
}

export function availableHeight({
    viewportHeight,
    scrollY,
    top,
    footnoteHeight = FOOTNOTE_HEIGHT,
    extraHeight = 0,
    minHeight = MIN_CONTENT_HEIGHT,
}: AvailableHeightOptions): number {
    const remaining =
        viewportHeight - scrollY - top - footnoteHeight + extraHeight;

    return Math.max(Math.round(remaining), minHeight);
}
