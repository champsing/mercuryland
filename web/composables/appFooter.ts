import type { InjectionKey, Ref } from "vue";

/**
 * Measured height of the app footer in CSS pixels, provided by `App.vue`.
 *
 * The footer is a single row on desktop and two rows on mobile, so layouts
 * that size themselves against the viewport need the real height instead of
 * the hard-coded desktop value.
 */
export const APP_FOOTER_HEIGHT: InjectionKey<Ref<number>> =
    Symbol("app-footer-height");

/**
 * Distance from the footer's top edge to the bottom of the copyright line.
 *
 * On phones the four footer actions sit in a 2x2 grid below the copyright
 * line. Sizing a page's scroll area against this strip keeps the copyright at
 * the bottom of the viewport and pushes the actions below the fold, where they
 * are revealed by scrolling the page.
 */
export const APP_FOOTER_COPYRIGHT_HEIGHT: InjectionKey<Ref<number>> = Symbol(
    "app-footer-copyright-height",
);
